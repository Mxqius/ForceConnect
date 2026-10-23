//! Passing open sockets from the agent to the interface.
//!
//! Binding port 53 needs root on Linux, and the interface process does not
//! have it. Running the resolver inside the agent would put a network-facing
//! DNS server in the root process, which is exactly the kind of surface this
//! crate exists to keep small. So the agent binds the sockets and hands them
//! over: a file descriptor sent with `SCM_RIGHTS` is the same open socket in
//! the receiving process, already bound, with nothing more for root to do.
//!
//! Every handover is one byte, [`MARKER`], carrying zero or more descriptors.
//! The byte is sent even when there are none, so the receiver always reads
//! exactly one before the JSON reply line — and reads it with `recvmsg`, not
//! through the buffered reader, which would discard the descriptors.

use std::io;
use std::mem;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};
use std::os::unix::net::UnixStream;
use std::ptr;

const MARKER: u8 = b'S';

/// Descriptors one handover may carry: UDP and TCP, for each address family.
pub const MAX_FDS: usize = 4;

/// Room for `MAX_FDS` descriptors and the header in front of them, in `u64`
/// words so the buffer is aligned for `cmsghdr`.
type Control = [u64; 8];

#[cfg(target_os = "linux")]
const SEND_FLAGS: libc::c_int = libc::MSG_NOSIGNAL;
#[cfg(not(target_os = "linux"))]
const SEND_FLAGS: libc::c_int = 0;

#[cfg(target_os = "linux")]
const RECV_FLAGS: libc::c_int = libc::MSG_CMSG_CLOEXEC;
#[cfg(not(target_os = "linux"))]
const RECV_FLAGS: libc::c_int = 0;

/// Send the marker byte, carrying `fds`.
pub fn send(stream: &UnixStream, fds: &[RawFd]) -> io::Result<()> {
    if fds.len() > MAX_FDS {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "too many sockets for one handover",
        ));
    }

    let byte = [MARKER];
    let mut iov = libc::iovec {
        iov_base: byte.as_ptr().cast_mut().cast(),
        iov_len: 1,
    };
    let mut control: Control = [0; 8];

    // SAFETY: an all-zero `msghdr` is a valid empty message.
    let mut msg: libc::msghdr = unsafe { mem::zeroed() };
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;

    if !fds.is_empty() {
        let payload = mem::size_of_val(fds) as libc::c_uint;
        // SAFETY: CMSG_SPACE and CMSG_LEN are arithmetic on their argument.
        let space = unsafe { libc::CMSG_SPACE(payload) } as usize;
        debug_assert!(space <= mem::size_of::<Control>());

        msg.msg_control = control.as_mut_ptr().cast();
        msg.msg_controllen = space as _;

        // SAFETY: `msg_control` points at `control`, which is aligned and
        // large enough for one header and `fds`; CMSG_FIRSTHDR is therefore
        // non-null and CMSG_DATA points inside it.
        unsafe {
            let cmsg = libc::CMSG_FIRSTHDR(&msg);
            (*cmsg).cmsg_level = libc::SOL_SOCKET;
            (*cmsg).cmsg_type = libc::SCM_RIGHTS;
            (*cmsg).cmsg_len = libc::CMSG_LEN(payload) as _;
            ptr::copy_nonoverlapping(
                fds.as_ptr(),
                libc::CMSG_DATA(cmsg).cast::<RawFd>(),
                fds.len(),
            );
        }
    }

    loop {
        // SAFETY: `msg` and everything it points at outlive the call.
        let sent = unsafe { libc::sendmsg(stream.as_raw_fd(), &msg, SEND_FLAGS) };
        match sent {
            1 => return Ok(()),
            -1 if io::Error::last_os_error().kind() == io::ErrorKind::Interrupted => {}
            -1 => return Err(io::Error::last_os_error()),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "the handover byte was not sent",
                ));
            }
        }
    }
}

/// Read the marker byte and take ownership of the descriptors it carries.
pub fn receive(stream: &UnixStream) -> io::Result<Vec<OwnedFd>> {
    let mut byte = [0u8; 1];
    let mut iov = libc::iovec {
        iov_base: byte.as_mut_ptr().cast(),
        iov_len: 1,
    };
    let mut control: Control = [0; 8];

    // SAFETY: an all-zero `msghdr` is a valid empty message.
    let mut msg: libc::msghdr = unsafe { mem::zeroed() };
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_control = control.as_mut_ptr().cast();
    msg.msg_controllen = mem::size_of::<Control>() as _;

    let received = loop {
        // SAFETY: `msg` and everything it points at outlive the call.
        let received = unsafe { libc::recvmsg(stream.as_raw_fd(), &mut msg, RECV_FLAGS) };
        if received == -1 && io::Error::last_os_error().kind() == io::ErrorKind::Interrupted {
            continue;
        }
        break received;
    };
    if received == -1 {
        return Err(io::Error::last_os_error());
    }

    // Owned before anything else is checked, so that every error below closes
    // what arrived instead of leaking it into this process.
    let mut fds = Vec::new();
    // SAFETY: the kernel filled `control` and set `msg_controllen`; the CMSG
    // macros walk only within it, and every SCM_RIGHTS payload is a run of
    // descriptors this process now owns.
    unsafe {
        let mut cmsg = libc::CMSG_FIRSTHDR(&msg);
        while !cmsg.is_null() {
            if (*cmsg).cmsg_level == libc::SOL_SOCKET && (*cmsg).cmsg_type == libc::SCM_RIGHTS {
                let data = libc::CMSG_DATA(cmsg).cast::<RawFd>();
                let len = (*cmsg).cmsg_len as usize - libc::CMSG_LEN(0) as usize;
                for i in 0..len / mem::size_of::<RawFd>() {
                    fds.push(OwnedFd::from_raw_fd(ptr::read_unaligned(data.add(i))));
                }
            }
            cmsg = libc::CMSG_NXTHDR(&msg, cmsg);
        }
    }

    if received == 0 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "the privileged agent exited",
        ));
    }
    if msg.msg_flags & libc::MSG_CTRUNC != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "more sockets were sent than one handover allows",
        ));
    }
    if byte[0] != MARKER {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "the agent's reply did not start with a socket handover",
        ));
    }

    // Linux sets close-on-exec atomically through MSG_CMSG_CLOEXEC; elsewhere
    // it is set here, so a later spawn does not inherit port 53.
    #[cfg(not(target_os = "linux"))]
    for fd in &fds {
        // SAFETY: `fd` is a descriptor this process owns.
        if unsafe { libc::fcntl(fd.as_raw_fd(), libc::F_SETFD, libc::FD_CLOEXEC) } == -1 {
            return Err(io::Error::last_os_error());
        }
    }

    Ok(fds)
}

#[cfg(test)]
mod tests {
    use std::net::{TcpListener, UdpSocket};

    use super::*;

    #[test]
    fn a_socket_arrives_bound_to_the_same_address() {
        let (agent, interface) = UnixStream::pair().unwrap();
        let udp = UdpSocket::bind("127.0.0.1:0").unwrap();
        let tcp = TcpListener::bind("127.0.0.1:0").unwrap();

        send(&agent, &[udp.as_raw_fd(), tcp.as_raw_fd()]).unwrap();
        let mut fds = receive(&interface).unwrap().into_iter();

        let received_udp = UdpSocket::from(fds.next().unwrap());
        let received_tcp = TcpListener::from(fds.next().unwrap());
        assert!(fds.next().is_none());
        assert_eq!(
            received_udp.local_addr().unwrap(),
            udp.local_addr().unwrap()
        );
        assert_eq!(
            received_tcp.local_addr().unwrap(),
            tcp.local_addr().unwrap()
        );
    }

    #[test]
    fn a_handover_with_nothing_in_it_still_reads_one_byte() {
        // The refusal path: the agent sends the marker alone, then its error
        // line. The JSON after it must be left for the line reader.
        use std::io::{Read, Write};

        let (mut agent, mut interface) = UnixStream::pair().unwrap();
        send(&agent, &[]).unwrap();
        agent.write_all(b"{}\n").unwrap();

        assert!(receive(&interface).unwrap().is_empty());
        let mut rest = [0u8; 3];
        interface.read_exact(&mut rest).unwrap();
        assert_eq!(&rest, b"{}\n");
    }

    #[test]
    fn more_than_a_handover_holds_is_refused_before_sending() {
        let (agent, _interface) = UnixStream::pair().unwrap();
        let fds = [0; MAX_FDS + 1];
        assert!(send(&agent, &fds).is_err());
    }
}
