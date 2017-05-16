use super::{Path, Socket};
use std::net;

pub struct ConnectionCtx<E> {
    local_addr: net::SocketAddr,
    peer_addr: net::SocketAddr,
    dst_name: Path,
    endpoint: E,
}
impl<E> ConnectionCtx<E> {
    pub fn new(local: net::SocketAddr,
               peer: net::SocketAddr,
               dst: Path,
               ep: E)
               -> ConnectionCtx<E> {
        ConnectionCtx {
            local_addr: local,
            peer_addr: peer,
            dst_name: dst,
            endpoint: ep,
        }
    }

    pub fn local_addr(&self) -> net::SocketAddr {
        self.local_addr
    }

    pub fn peer_addr(&self) -> net::SocketAddr {
        self.peer_addr
    }

    pub fn dst_name(&self) -> &Path {
        &self.dst_name
    }

    pub fn endpoint(&self) -> &E {
        &self.endpoint
    }
}

/// A src or dst connection.
pub struct Connection<E> {
    pub context: ConnectionCtx<E>,
    pub socket: Socket,
}
impl<E> Connection<E> {
    pub fn peer_addr(&self) -> net::SocketAddr {
        self.context.peer_addr
    }
    pub fn local_addr(&self) -> net::SocketAddr {
        self.context.local_addr
    }
}
