#!/usr/bin/env bash

bindgen \
    --no-doc-comments \
    --no-layout-tests \
    --no-prepend-enum-name \
    --size_t-is-usize \
    --default-enum-style=rust \
    include/cadef.h -o sys.rs \
    -- \
    -I include \
    -I include/compiler/clang \
    -I include/os/Linux
