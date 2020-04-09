use addr_hal::{SocketAddressV4, SocketAddressV6, SocketAddr};
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
    async fn connect(&self, addr: &[SocketAddr<Self::SA4, Self::SA6>]) -> Result<(), Self::Error>;

    /// Send data to remote.
    async fn send(&mut self, buffer: &[u8]) -> Result<usize, Self::Error>;

    /// Recv data from remote.
    async fn recv(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error>;
}

#[async_trait]
pub trait UdpServer {
    type SA4: SocketAddressV4;

    type SA6: SocketAddressV6;

    type Error;

    type BindResult: UdpSocket;

    async fn bind(addr: &[SocketAddr<Self::SA4, Self::SA6>]) -> Result<Self::BindResult, Self::Error>;
}
