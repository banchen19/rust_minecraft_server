use bds_sdk::{BdsNetWork, Bds_NetWork};

extern crate bds_sdk;

#[tokio::main]
async fn main() {
    let ip_str = "0.0.0.0";
    let port_str = "8080";

    // 调用Bds_NetWork::start方法
    let bds: Bds_NetWork = Bds_NetWork::start(ip_str, port_str).await;
    let mut listener = bds.get_listener().await;
    loop {
        let socket = listener.accept().await.unwrap();
        listener.get_motd().await;
        let buf = socket.recv().await.unwrap();
        if buf[0] == 0xfe {
            let mut buffer: Vec<u8> = Vec::new();
            // buffer.
        }
    }
}
