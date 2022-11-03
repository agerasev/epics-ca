#include <cadef.h>

void test_set_ca_access_rights(caar *none, caar *read, caar *write, caar *all)
{
    *none = (caar){
        .read_access = 0,
        .write_access = 0,
    };
    *read = (caar){
        .read_access = 1,
        .write_access = 0,
    };
    *write = (caar){
        .read_access = 0,
        .write_access = 1,
    };
    *all = (caar){
        .read_access = 1,
        .write_access = 1,
    };
}
