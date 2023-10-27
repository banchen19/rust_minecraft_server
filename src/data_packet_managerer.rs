use std::collections::HashMap;
use crate::Data_Pack::*;
trait DataPacketProcessor {
    fn get_protocol(&self) -> i32;
    fn get_packet_id(&self) -> i32;
    fn handle(&self, player_handle: &PlayerHandle, packet: &DataPacket);
}   

struct DataPacketManager {
    current_protocol_processors: HashMap<i32, Box<dyn DataPacketProcessor>>,
}

impl DataPacketManager {
    fn new() -> Self {
        DataPacketManager {
            current_protocol_processors: HashMap::new(),
        }
    }

    fn register_processor(&mut self, processors: Vec<Box<dyn DataPacketProcessor>>) {
        for processor in processors {
            if processor.get_protocol() != ProtocolInfo::CURRENT_PROTOCOL {
                panic!("Processor protocol {} does not match current protocol {}. Multi-version support is not implemented yet.", processor.get_protocol(), ProtocolInfo::CURRENT_PROTOCOL);
            }
            self.current_protocol_processors.insert(processor.get_packet_id(), processor);
        }
    }

    fn can_process(&self, protocol: i32, packet_id: i32) -> bool {
        if protocol != ProtocolInfo::CURRENT_PROTOCOL {
            return false;
        }
        self.current_protocol_processors.contains_key(&packet_id)
    }

    fn process_packet(&self, player_handle: &PlayerHandle, packet: &DataPacket) {
        if packet.get_protocol_used() != ProtocolInfo::CURRENT_PROTOCOL {
            panic!("Packet protocol {} does not match current protocol {}. Multi-version support is not implemented yet.", packet.get_protocol_used(), ProtocolInfo::CURRENT_PROTOCOL);
        }
        match self.current_protocol_processors.get(&packet.get_packet_id()) {
            Some(processor) => {
                processor.handle(player_handle, packet);
            }
            None => {
                panic!("No processor found for packet {} with id {}.", std::any::type_name::<DataPacket>(), packet.get_packet_id());
            }
        }
    }

    fn register_default_processors(&mut self) {
        let processors: Vec<Box<dyn DataPacketProcessor>> = vec![
            Box::new(RequestNetworkSettingsProcessor),
            Box::new(LoginProcessor),
            // Add other processor instances here...
        ];
        self.register_processor(processors);
    }
}

impl ProtocolInfo {
    const CURRENT_PROTOCOL: i32 = 1; // Replace with the actual protocol value.
}
