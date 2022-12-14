use super::{GetFn, GetState, GetToSlice, GetVec, Put, ScalarChannel, TypedChannel};
use crate::{
    error::Error,
    types::{
        request::{Array, Meta, Scalar, ScalarRequest, Time, TypedRequest},
        EpicsTimeStamp, Field,
    },
};
use futures::stream::{select, Stream, TryStreamExt};
use pin_project::pin_project;
use std::{
    ffi::CString,
    mem,
    pin::Pin,
    task::{Context, Poll},
};

impl<T: Field> TypedChannel<T> {
    pub async fn into_array(self) -> Result<ArrayChannel<T>, Error> {
        ArrayChannel::new(self).await
    }
}

#[derive(Debug)]
pub struct ArrayChannel<T: Field> {
    value: TypedChannel<T>,
    nord: ScalarChannel<f64>,
}

impl<T: Field> ArrayChannel<T> {
    pub async fn new(chan: TypedChannel<T>) -> Result<Self, Error> {
        let name = CString::from_vec_with_nul(
            chan.name()
                .to_bytes()
                .iter()
                .chain(b".NORD\0".iter())
                .copied()
                .collect(),
        )
        .unwrap();

        let nord = chan
            .context()
            .clone()
            .connect_typed::<f64>(&name)
            .await?
            .into_scalar()
            .map_err(|(e, _)| e)?;

        Ok(Self { value: chan, nord })
    }

    pub fn put(&mut self, data: &[T]) -> Result<Put<'_>, Error> {
        self.value.put_slice(data)
    }
}

impl<T: Field> ArrayChannel<T>
where
    Time: Meta<T>,
{
    pub async fn get_with<F: GetFn<Request = [T]>>(&mut self, func: F) -> Result<F::Output, Error> {
        let mut state = GetState::Pending(func);
        loop {
            let nord = self.nord.get_request::<Scalar<f64, Time>>().await?;
            self.value
                .get_request_with(GetArrayWith {
                    nord,
                    state: &mut state,
                })
                .await?;
            if let GetState::Ready(output) = state {
                break output;
            }
        }
    }

    pub async fn get_to_slice(&mut self, dst: &mut [T]) -> Result<usize, Error> {
        self.get_with(GetToSlice::from(dst)).await
    }

    pub async fn get_vec(&mut self) -> Result<Vec<T>, Error> {
        self.get_with(GetVec::default()).await
    }

    pub fn subscribe_vec(&mut self) -> impl Stream<Item = Result<Vec<T>, Error>> + '_ {
        let nord = self.nord.subscribe_request::<Scalar<f64, Time>>();
        let array = self.value.subscribe_request_vec::<Array<T, Time>>();
        SubscribeArray {
            stream: select(
                nord.map_ok(|x| LoaTs {
                    loa: LenOrArray::Len(*x.value() as usize),
                    ts: x.meta().stamp,
                }),
                array.map_ok(|(m, v)| LoaTs {
                    loa: LenOrArray::Array(v),
                    ts: m.stamp,
                }),
            ),
            last: None,
        }
    }
}

struct GetArrayWith<'a, T: Field, F: GetFn<Request = [T]>> {
    nord: Scalar<f64, Time>,
    state: &'a mut GetState<F>,
}

impl<'a, T: Field, F: GetFn<Request = [T]>> GetFn for GetArrayWith<'a, T, F>
where
    Time: Meta<T>,
{
    type Request = Array<T, Time>;
    type Output = ();

    fn apply(self, input: Result<&Self::Request, Error>) -> Result<Self::Output, Error> {
        let req = match input {
            Ok(req) => {
                let len = *self.nord.value() as usize;
                println!(
                    "nord: {}, timestamp: {:?}",
                    len,
                    self.nord.stamp.to_system()
                );
                if req.stamp != self.nord.stamp {
                    return Ok(());
                }
                Ok(&req.values()[..len])
            }
            Err(err) => Err(err),
        };
        let func = match mem::replace(self.state, GetState::Empty) {
            GetState::Pending(func) => func,
            _ => unreachable!(),
        };
        *self.state = GetState::Ready(func.apply(req));
        Ok(())
    }
}

#[derive(Clone, Debug)]
enum LenOrArray<T: Field> {
    Len(usize),
    Array(Vec<T>),
}

#[derive(Clone, Debug)]
struct LoaTs<T: Field> {
    loa: LenOrArray<T>,
    ts: EpicsTimeStamp,
}

#[pin_project]
struct SubscribeArray<T: Field, S: Stream<Item = Result<LoaTs<T>, Error>>> {
    #[pin]
    stream: S,
    last: Option<LoaTs<T>>,
}

impl<T: Field, S: Stream<Item = Result<LoaTs<T>, Error>>> Stream for SubscribeArray<T, S> {
    type Item = Result<Vec<T>, Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        // FIXME: Should poll stream in loop.
        let mut current = match this.stream.poll_next(cx) {
            Poll::Ready(Some(Ok(loats))) => {
                println!("LoaTs stream: {:?}", loats);
                loats
            }
            other => return other.map_ok(|_| unreachable!()),
        };
        if let Some(mut last) = this.last.take() {
            if last.ts > current.ts {
                *this.last = Some(last);
                return Poll::Pending;
            }
            if last.ts == current.ts {
                if let (LenOrArray::Len(len), LenOrArray::Array(vec))
                | (LenOrArray::Array(vec), LenOrArray::Len(len)) =
                    (&mut last.loa, &mut current.loa)
                {
                    vec.truncate(*len);
                    return Poll::Ready(Some(Ok(mem::take(vec))));
                }
            }
        }
        *this.last = Some(current);
        Poll::Pending
    }
}

#[cfg(test)]
mod tests {
    use crate::{Channel, Context};
    use async_std::test as async_test;
    use c_str_macro::c_str;
    use futures::{pin_mut, StreamExt};
    use serial_test::serial;

    #[async_test]
    #[serial]
    async fn put_get() {
        let ctx = Context::new().unwrap();

        let mut output = Channel::new(ctx.clone(), c_str!("ca:test:aao")).unwrap();
        output.connected().await;
        let mut output = output
            .into_typed::<i32>()
            .unwrap()
            .into_array()
            .await
            .unwrap();

        let mut input = Channel::new(ctx, c_str!("ca:test:aai")).unwrap();
        input.connected().await;
        let mut input = input
            .into_typed::<i32>()
            .unwrap()
            .into_array()
            .await
            .unwrap();

        let data = (0..8).into_iter().collect::<Vec<i32>>();
        output.put(&data).unwrap().await.unwrap();
        assert_eq!(input.get_vec().await.unwrap(), data);
    }

    #[async_test]
    #[serial]
    async fn subscribe() {
        let ctx = Context::new().unwrap();

        let mut output = Channel::new(ctx.clone(), c_str!("ca:test:aao")).unwrap();
        output.connected().await;
        let mut output = output
            .into_typed::<i32>()
            .unwrap()
            .into_array()
            .await
            .unwrap();

        let mut input = Channel::new(ctx, c_str!("ca:test:aai")).unwrap();
        input.connected().await;
        let mut input = input
            .into_typed::<i32>()
            .unwrap()
            .into_array()
            .await
            .unwrap();

        output.put(&[0]).unwrap().await.unwrap();
        let monitor = input.subscribe_vec();
        pin_mut!(monitor);
        assert_eq!(monitor.next().await.unwrap().unwrap(), [0]);

        output.put(&[]).unwrap().await.unwrap();
        assert_eq!(monitor.next().await.unwrap().unwrap(), []);

        /*
        let count = 0x10;
        join!(
            async {
                for i in 0..count {
                    output.put((i + 1) as f64 / 16.0).unwrap().await.unwrap();
                }
            },
            async {
                for i in 0..count {
                    assert_eq!(
                        monitor.next().await.unwrap().unwrap(),
                        (i + 1) as f64 / 16.0
                    );
                    sleep(Duration::from_millis(10)).await;
                }
            }
        );
        */
    }
}
