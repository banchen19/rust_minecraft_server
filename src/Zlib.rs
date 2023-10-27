use tokio::io;

struct Zlib{
    providers:Vec<u8>,
    provider:dyn ZlibProvider
}


trait ZlibProvider {
    fn deflate(&self, data: Vec<Vec<u8>>, level: i32) -> Result<Vec<u8>, io::Error>;
    fn deflate_single(&self, data: Vec<u8>, level: i32) -> Result<Vec<u8>, io::Error>;
    fn inflate(&self, data: Vec<u8>, max_size: usize) -> Result<Vec<u8>, io::Error>;
}


impl Zlib {
    fn set_provider(&mut self, provider_index: u32) {
        let lang = "eng"; // 请替换为实际的语言字符串
        match provider_index {
            0 => {
                if self.providers[provider_index as usize].is_none() {
                    self.providers[provider_index as usize] = Some(Box::new(ZlibOriginal));
                }
            }
            1 => {
                if self.providers[provider_index as usize].is_none() {
                    self.providers[provider_index as usize] = Some(Box::new(ZlibSingleThreadLowMem));
                }
            }
            2 => {
                if self.providers[provider_index as usize].is_none() {
                    self.providers[provider_index as usize] = Some(Box::new(ZlibThreadLocal));
                }
                if Libdeflate::is_available() {
                    println!("{} {}", "Zlib acceleration can enable", TextFormat::WHITE);
                }
            }
            3 => {
                if Libdeflate::is_available() {
                    Network::libdeflate_available = true;
                    if self.providers[provider_index as usize].is_none() {
                        self.providers[provider_index as usize] = Some(Box::new(LibDeflateThreadLocal));
                    }
                } else {
                    println!("{} {}", lang.tr("nukkit.zlib.unavailable"));
                    provider_index = 2;
                    if self.providers[provider_index as usize].is_none() {
                        self.providers[provider_index as usize] = Some(Box::new(ZlibThreadLocal));
                    }
                }
            }
            _ => {
                panic!("Invalid provider: {}", provider_index);
            }
        }

        if provider_index < 2 {
            println!("{}", lang.tr("nukkit.zlib.affect-performance"));
        }
        if provider_index == 3 {
            println!("{}", lang.tr("nukkit.zlib.acceleration-experimental"));
        }

        self.provider = self.providers[provider_index as usize].take();

        println!(
            "{}: {} ({})",
            lang.tr("nukkit.zlib.selected"),
            provider_index,
            self.provider
                .as_ref()
                .map(|p| p.name())
                .unwrap_or("None")
        );
    }
}