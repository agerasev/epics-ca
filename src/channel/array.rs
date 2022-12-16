use super::{
    get::Callback,
    subscribe::{LastFn, Queue},
    Channel, Get, GetFn, Put, Subscription,
};
use crate::{
    error::{self, Error},
    request::{ReadRequest, Request, TypedRequest, WriteRequest},
    types::Field,
};
use derive_more::{Deref, DerefMut, Into};
use std::{
    any::type_name,
    fmt::{self, Debug},
    marker::PhantomData,
    ptr,
};

impl Channel {
    pub fn into_array<T: Field>(self) -> Result<ArrayChannel<T>, (Error, Self)> {
        let dbf = match self.field_type() {
            Ok(dbf) => dbf,
            Err(err) => return Err((err, self)),
        };
        if dbf == T::ID {
            Ok(ArrayChannel::new_unchecked(self))
        } else {
            Err((error::BADTYPE, self))
        }
    }
}

/// Typed channel.
#[repr(transparent)]
#[derive(Deref, DerefMut, Into)]
pub struct ArrayChannel<T: Field> {
    #[deref]
    #[deref_mut]
    pub(crate) base: Channel,
    #[into(ignore)]
    _p: PhantomData<T>,
}

impl<T: Field> ArrayChannel<T> {
    /// Convert [`Channel`] to [`ArrayChannel<T>`] without type checking.
    ///
    /// It is safe because the type of remote channel can change at any moment and checks are done during reading/writing/monitoring anyway.
    ///
    /// If you want to check type before converting use [`AnyChannel::into_array`].
    pub fn new_unchecked(base: Channel) -> Self {
        Self {
            base,
            _p: PhantomData,
        }
    }
}

pub(crate) struct ProcessData {
    id_counter: usize,
    pub(crate) data: *mut u8,
    pub(crate) put_res: Option<Result<(), Error>>,
}

impl ProcessData {
    pub fn new() -> Self {
        Self {
            id_counter: 0,
            data: ptr::null_mut(),
            put_res: None,
        }
    }
    pub fn id(&self) -> usize {
        self.id_counter
    }
    pub fn change_id(&mut self) {
        self.id_counter += 1;
    }
}

impl<T: Field> Debug for ArrayChannel<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Channel<{}>({:?})", type_name::<T>(), self.raw())
    }
}

impl<T: Field> ArrayChannel<T> {
    pub fn put<R>(&mut self, req: &R) -> Result<Put<'_>, Error>
    where
        R: TypedRequest<Value = [T]> + WriteRequest + ?Sized,
    {
        self.base.put::<R>(req)
    }

    pub fn get_with<R, F>(&mut self, func: F) -> Get<'_, F>
    where
        R: TypedRequest<Value = [T]> + ReadRequest + ?Sized,
        F: Callback<Request = R>,
    {
        self.base.get_with(func)
    }

    pub fn get_boxed<R>(&mut self) -> Get<'_, GetFn<R, Box<R>>>
    where
        R: TypedRequest<Value = [T]> + ReadRequest + ?Sized,
    {
        self.get_with(GetFn::<R, Box<R>>::new(clone_boxed::<R>))
    }

    pub fn get_to_slice<'a, 'b>(&'a mut self, dst: &'b mut [T]) -> Get<'a, GetToSlice<'b, T>> {
        self.get_with(GetToSlice { dst })
    }

    pub fn subscribe_with<F: Queue>(&mut self, func: F) -> Subscription<'_, F>
    where
        F::Request: TypedRequest<Value = [T]> + ReadRequest,
    {
        Subscription::new(self, func)
    }

    pub fn subscribe_boxed<R>(&mut self) -> Subscription<'_, LastFn<R, Box<R>>>
    where
        R: TypedRequest<Value = [T]> + ReadRequest + ?Sized,
    {
        self.subscribe_with(LastFn::<R, Box<R>>::new(clone_boxed_some::<R>))
    }
}

fn clone_boxed<R: Request + ?Sized>(input: Result<&R, Error>) -> Result<Box<R>, Error> {
    input.map(|req| req.clone_boxed())
}

fn clone_boxed_some<R: Request + ?Sized>(
    input: Result<&R, Error>,
) -> Option<Result<Box<R>, Error>> {
    Some(input.map(|req| req.clone_boxed()))
}

pub struct GetToSlice<'a, T: Field> {
    dst: &'a mut [T],
}

impl<'a, T: Field> Callback for GetToSlice<'a, T> {
    type Request = [T];
    type Output = usize;
    fn apply(self, input: Result<&[T], Error>) -> Result<Self::Output, Error> {
        input.map(|src| {
            let len = usize::min(self.dst.len(), src.len());
            self.dst[..len].copy_from_slice(&src[..len]);
            src.len()
        })
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
    async fn downcast() {
        let ctx = Context::new().unwrap();
        let mut base = Channel::new(ctx, c_str!("ca:test:ai")).unwrap();
        base.connected().await;
        base.into_array::<f64>().unwrap();
    }

    #[async_test]
    #[serial]
    async fn put_get() {
        let ctx = Context::new().unwrap();

        let mut output = Channel::new(ctx.clone(), c_str!("ca:test:aao")).unwrap();
        output.connected().await;
        let mut output = output.into_array::<i32>().unwrap();

        let mut input = Channel::new(ctx, c_str!("ca:test:aai")).unwrap();
        input.connected().await;
        let mut input = input.into_array::<i32>().unwrap();

        let data = (0..8).into_iter().collect::<Vec<i32>>();
        output.put::<[i32]>(&data).unwrap().await.unwrap();
        assert_eq!(Vec::from(input.get_boxed().await.unwrap()), data);
    }

    #[async_test]
    #[serial]
    async fn subscribe() {
        let ctx = Context::new().unwrap();

        let mut output = Channel::new(ctx.clone(), c_str!("ca:test:aao")).unwrap();
        output.connected().await;
        let mut output = output.into_array::<i32>().unwrap();

        let mut input = Channel::new(ctx, c_str!("ca:test:aai")).unwrap();
        input.connected().await;
        let mut input = input.into_array::<i32>().unwrap();

        output.put::<[i32]>(&[-1]).unwrap().await.unwrap();
        let monitor = input.subscribe_boxed();
        pin_mut!(monitor);
        assert_eq!(Vec::from(monitor.next().await.unwrap().unwrap()), [-1]);

        let count = 0x10;
        for i in 0..count {
            let data = (0..(i + 1)).collect::<Vec<_>>();
            output.put::<[i32]>(&data).unwrap().await.unwrap();
            assert_eq!(Vec::from(monitor.next().await.unwrap().unwrap()), data);
        }
    }
}
