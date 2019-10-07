#![allow(non_camel_case_types)]
use crate::ctx::WasiCtx;
use crate::wasm32;

hostcalls! {
    pub unsafe fn test_func(
        _wasi_ctx: &WasiCtx,
        memory: &mut [u8],
        tin: wasm32::size_t,
        tout: wasm32::uintptr_t,
    ) -> wasm32::__wasi_errno_t;

    pub unsafe fn sock_create(
        wasi_ctx: &mut WasiCtx,
        memory: &mut [u8],
        sock: wasm32::uintptr_t,
        domain: wasm32::size_t,
        type_: wasm32::size_t,
    ) -> wasm32::__wasi_errno_t;

    pub unsafe fn sock_connect(
        wasi_ctx: &mut WasiCtx,
        memory: &[u8],
        sock: wasm32::__wasi_fd_t,
        domain: wasm32::size_t,
        addr: wasm32::uintptr_t,
        port: u16,
    ) -> wasm32::__wasi_errno_t;
}
