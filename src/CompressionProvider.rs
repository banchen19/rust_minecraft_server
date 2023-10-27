trait CompressionProvider {
    fn compress(&self, buffer: Vec<u8>, level: u32) -> Vec<u8>;
    fn decompress(&self, buffer: Vec<u8>) -> Vec<u8>;
}

enum PacketCompressionAlgorithm {
    ZLIB,
    SNAPPY,
    NONE,
}

struct NoneCompressionProvider;

impl CompressionProvider for NoneCompressionProvider {
    fn compress(&self, buffer: Vec<u8>, level: u32) -> Vec<u8> {
        buffer
    }

    fn decompress(&self, buffer: Vec<u8>) -> Vec<u8> {
        buffer
    }
}

struct SnappyCompressionProvider;

impl CompressionProvider for SnappyCompressionProvider {
    fn compress(&self, buffer: Vec<u8>, level: u32) -> Vec<u8> {
        // 使用 Snappy 压缩逻辑
        // 用适当的代码替换 unimplemented!()
        // 示例：snappy::compress(buffer)
        unimplemented!()
    }

    fn decompress(&self, buffer: Vec<u8>) -> Vec<u8> {
        // 使用 Snappy 解压逻辑
        // 用适当的代码替换 unimplemented!()
        // 示例：snappy::decompress(buffer)
        unimplemented!()
    }
}

struct ZlibCompressionProvider;

impl CompressionProvider for ZlibCompressionProvider {
    fn compress(&self, buffer: Vec<u8>, level: u32) -> Vec<u8> {
        // 使用 Zlib 压缩逻辑
        // 用适当的代码替换 unimplemented!()
        unimplemented!()
    }

    fn decompress(&self, buffer: Vec<u8>) -> Vec<u8> {
        // 使用 Zlib 解压逻辑
        // 用适当的代码替换 unimplemented!()
        unimplemented!()
    }
}

impl dyn CompressionProvider {
    fn from(algorithm: PacketCompressionAlgorithm) -> Box<dyn CompressionProvider> {
        match algorithm {
            PacketCompressionAlgorithm::NONE => Box::new(NoneCompressionProvider),
            PacketCompressionAlgorithm::ZLIB => Box::new(ZlibCompressionProvider),
            PacketCompressionAlgorithm::SNAPPY => Box::new(SnappyCompressionProvider),
        }
    }
}
