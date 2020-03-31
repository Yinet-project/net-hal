use addr_hal::{ToSocketAddrs, SocketAddressV4, SocketAddressV6};
use async_trait::async_trait;

#[async_trait]
pub trait UdpHandler {
    /// need specific type for SocketAddressV4.
    type SA4: SocketAddressV4;

    /// need specific type for SocketAddressV4.
    type SA6: SocketAddressV6;
    
    /// Error Type.
    type Error;

    /// Connect to remote peer.
    async fn connect<A>(addr: A) where A: ToSocketAddrs<Self::SA4, Self::SA6>;

    /// Send data to remote.
    async fn send<B>(buffer: B) -> Result<(), Self::Error>;

    // Recv data from remote.
    async fn recv<B>() -> Result<B, Self::Error>;
}

#[async_trait]
pub trait UdpServer {
    type SA4: SocketAddressV4;

    type SA6: SocketAddressV6;

    type Error;

    async fn bind<A>(addr: A) where A: ToSocketAddrs<Self::SA4, Self::SA6>;
}

