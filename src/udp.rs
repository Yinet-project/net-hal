use addr_hal::{ToSocketAddrs, SocketAddressV4, SocketAddressV6};
use core::convert::AsRef;

pub trait UdpHandler {
    /// need specific type for SocketAddressV4.
    type SA4: SocketAddressV4;

    /// need specific type for SocketAddressV4.
    type SA6: SocketAddressV6;
    
    /// Error Type.
    type Error;

    /// Connect to remote peer.
    fn connect<A>(addr: A) where A: ToSocketAddrs<Self::SA4, Self::SA6>;

    /// Send data to remote.
    fn send<B>(buffer: B) -> Result<(), Self::Error> where B: AsRef<[u8]>;

    /// Recv data from remote.
    fn recv<B>() -> Result<B, Self::Error> where B: AsRef<[u8]>;
}

pub trait UdpServer {
    type SA4: SocketAddressV4;

    type SA6: SocketAddressV6;

    type Error;

    fn bind<A>(addr: A) where A: ToSocketAddrs<Self::SA4, Self::SA6>;
}

