use crate::constant::InboundListener;
use crate::tunnel::Tunnel;

mod listener;
mod tunnel;
mod constant;
mod adapter;
mod errors;
mod component;

#[tokio::main]
async fn main() {
    let tunnel = Tunnel::new();
    let socks5_listener = listener::inbound::socks::Socks::new();
    println!("running socks5 listener on :7892");
    socks5_listener.listen(&tunnel).await.unwrap();
    println!("socks5 listener stopped");
}
