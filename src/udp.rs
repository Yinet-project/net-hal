use addr_hal::{SocketAddressV4, SocketAddressV6, ToSocketAddrs};
use async_trait::async_trait;

#[async_trait]
pub trait UdpSocket {
    /// need specific type for SocketAddressV4.
    type SA4: SocketAddressV4;

    /// need specific type for SocketAddressV4.
    type SA6: SocketAddressV6;

    /// Error Type.
    type Error;

    /// Connect to remote peer.
    async fn connect<A>(&self, addr: A) -> Result<(), Self::Error>
    where
        A: ToSocketAddrs<Self::SA4, Self::SA6>;

    /// Send data to remote.
    async fn send<B>(&self, buffer: B) -> Result<(), Self::Error>;

    /// Recv data from remote.
    async fn recv<B>(&self) -> Result<B, Self::Error>;
}

#[async_trait]
pub trait UdpServer {
    type SA4: SocketAddressV4;

    type SA6: SocketAddressV6;

    type Error;

    type BindResult: UdpSocket;

    async fn bind<A>(addr: A) -> Result<Self::BindResult, Self::Error>
    where
        A: ToSocketAddrs<Self::SA4, Self::SA6>;
}
