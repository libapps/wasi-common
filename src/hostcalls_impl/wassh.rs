#![allow(non_camel_case_types)]
use crate::ctx::WasiCtx;
use crate::fdentry::FdEntry;
use crate::memory::*;
use crate::sys::host_impl;
use crate::{wasm32, Error, Result};
use log::trace;
use std::convert::TryInto;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::os::unix::prelude::FromRawFd;

pub(crate) unsafe fn test_func(
    _wasi_ctx: &WasiCtx,
    memory: &mut [u8],
    tin: wasm32::size_t,
    tout: wasm32::uintptr_t,
) -> Result<()> {
    trace!("test_func(tin={:?}, tout={:#?})", tin, tout);
    enc_usize_byref(memory, tout, ((tin + 10) | 0x80).try_into().unwrap())
}

pub(crate) unsafe fn sock_create(
    wasi_ctx: &mut WasiCtx,
    memory: &mut [u8],
    sock: wasm32::uintptr_t,
    domain: wasm32::size_t,
    type_: wasm32::size_t,
) -> Result<()> {
    trace!(
        "sock_create(sock={:#x}, domain={:?}, type={:?})",
        sock,
        domain,
        type_
    );
    let c_domain = match domain {
        // These come from __header_sys_socket.h.
        1 => libc::AF_INET,
        2 => libc::AF_INET6,
        3 => libc::AF_UNIX,
        _ => return Err(Error::EINVAL),
    };
    let c_type = match type_ {
        host::__WASI_FILETYPE_SOCKET_DGRAM => libc::SOCK_DGRAM,
        host::__WASI_FILETYPE_SOCKET_STREAM => libc::SOCK_STREAM,
        _ => return Err(Error::EINVAL),
    };

    let fd = libc::socket(c_domain, c_type, 0);
    if fd < 0 {
        return Err(host_impl::errno_from_nix(nix::errno::Errno::last()));
    }
    let file = File::from_raw_fd(fd);

    let fe = FdEntry::from(file)?;
    let guest_fd = wasi_ctx.insert_fd_entry(fe)?;

    trace!("     | *sock={:?}", guest_fd);

    enc_fd_byref(memory, sock, guest_fd.try_into().unwrap())
}

#[repr(C)]
union super_sockaddr {
    storage: libc::sockaddr_storage,
    sockaddr: libc::sockaddr,
    sockaddr_in: libc::sockaddr_in,
    sockaddr_in6: libc::sockaddr_in6,
}

pub(crate) unsafe fn sock_connect(
    wasi_ctx: &mut WasiCtx,
    memory: &[u8],
    sock: wasm32::__wasi_fd_t,
    domain: wasm32::size_t,
    addr: wasm32::uintptr_t,
    port: u16,
) -> Result<()> {
    trace!(
        "sock_connect(sock={:?}, domain={:?}, addr={:#x}, port={:?})",
        sock,
        domain,
        addr,
        port
    );
    let addr = match domain {
        1 => {  // AF_INET
            let c_addr = *dec_ptr_to(memory, addr)?;
            super_sockaddr {
                sockaddr_in: libc::sockaddr_in {
                    sin_family: libc::AF_INET as u16,
                    sin_port: port,
                    sin_addr: libc::in_addr { s_addr: c_addr },
                    sin_zero: [0; 8],
                },
            }
        }
        2 => {  // AF_INET6
            let c_addr = *dec_ptr_to(memory, addr)?;
            super_sockaddr {
                sockaddr_in6: libc::sockaddr_in6 {
                    sin6_family: libc::AF_INET6 as u16,
                    sin6_port: port,
                    sin6_addr: c_addr,
                    sin6_flowinfo: 0,
                    sin6_scope_id: 0,
                },
            }
        }
        // TODO: Missing AF_UNIX
        _ => return Err(Error::EINVAL)
    };

    let fe = wasi_ctx.get_fd_entry(sock, 0, 0)?;
let fd = fe.fd_object.descriptor.as_file()?.as_raw_fd();

    let res = libc::connect(
        fd.try_into().unwrap(),
        &addr.sockaddr,
        std::mem::size_of_val(&addr) as _,
    );
    if res < 0 {
        Err(host_impl::errno_from_nix(nix::errno::Errno::last()))
    } else {
        Ok(())
    }
    //  libc::connect(sock,
}
