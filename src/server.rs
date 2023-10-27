
use rust_raknet::RaknetListener;

use crate::DataPacket;

pub struct BdsNetWork {
    motd: String,
    raknet_listener: RaknetListener,
}
impl BdsNetWork {
    async fn start(ip_str: &str, port_str: &str) -> Self {
        // 解析字符串为IpAddr
        // let ip_addr = IpAddr::V4(Ipv4Addr::from_str(ip_str).expect("Invalid IP address"));
        // // 解析字符串为u16端口号
        // let port = u16::from_str(port_str).expect("Invalid port number");
        // let mut ip_addr = SocketAddr::new(ip_addr, port);
        let mut server_listener = RaknetListener::bind(&format!("{}:{}", ip_str, port_str))
            .await
            .expect("端口被占用");
        server_listener.listen().await;
        BdsNetWork {
            motd: server_listener.get_motd().await,
            raknet_listener: server_listener,
        }
    }
    async fn get_listener(self) -> RaknetListener {
        self.raknet_listener
    }
    async fn get_motd(self) -> String {
        self.raknet_listener.get_motd().await
    }
    // async fn processBatch(self,packets:DataPacket,compression:CompressionProvider)
    // {
        
    // }
}
