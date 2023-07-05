#[derive(Debug)]
struct TcpHeader {
    source_port: u16,
    destination_port: u16,
    sequence_number: u32,
    acknowledgement_number: u32,
    data_offset: u8,
    reserved: u8,
    flags: u16,
    window_size: u16,
    checksum: u16,
    urgent_pointer: u16,
    options: Vec<u8>,
}

impl TcpHeader {
    fn from_bytes(bytes: &[u8]) -> TcpHeader {
        let mut iter = bytes.iter();

        let source_port = Self::u16_from_iter(&mut iter);
        let destination_port = Self::u16_from_iter(&mut iter);
        let sequence_number = Self::u32_from_iter(&mut iter);
        let acknowledgement_number = Self::u32_from_iter(&mut iter);

        let data_offset_and_reserved_and_flags = Self::u16_from_iter(&mut iter);
        let data_offset = (data_offset_and_reserved_and_flags >> 12) as u8;
        let reserved = ((data_offset_and_reserved_and_flags & 0b0000_0111_0000_0000) >> 9) as u8;
        let flags = data_offset_and_reserved_and_flags & 0b0000_0000_1_1111_1111;

        let window_size = Self::u16_from_iter(&mut iter);
        let checksum = Self::u16_from_iter(&mut iter);
        let urgent_pointer = Self::u16_from_iter(&mut iter);

        let options = iter.cloned().collect::<Vec<_>>();

        TcpHeader {
            source_port,
            destination_port,
            sequence_number,
            acknowledgement_number,
            data_offset,
            reserved,
            flags,
            window_size,
            checksum,
            urgent_pointer,
            options,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(&self.source_port.to_be_bytes());
        bytes.extend(&self.destination_port.to_be_bytes());
        bytes.extend(&self.sequence_number.to_be_bytes());
        bytes.extend(&self.acknowledgement_number.to_be_bytes());

        let data_offset_and_reserved_and_flags =
            ((self.data_offset as u16) << 12) | ((self.reserved as u16) << 9) | self.flags;
        bytes.extend(&data_offset_and_reserved_and_flags.to_be_bytes());

        bytes.extend(&self.window_size.to_be_bytes());
        bytes.extend(&self.checksum.to_be_bytes());
        bytes.extend(&self.urgent_pointer.to_be_bytes());
        bytes.extend(&self.options);

        bytes
    }

    fn u16_from_iter(iter: &mut std::slice::Iter<'_, u8>) -> u16 {
        let b1 = *iter.next().unwrap() as u16;
        let b2 = *iter.next().unwrap() as u16;

        (b1 << 8) | b2
    }

    fn u32_from_iter(iter: &mut std::slice::Iter<'_, u8>) -> u32 {
        let b1 = *iter.next().unwrap() as u32;
        let b2 = *iter.next().unwrap() as u32;
        let b3 = *iter.next().unwrap() as u32;
        let b4 = *iter.next().unwrap() as u32;

        (b1 << 24) | (b2 << 16) | (b3 << 8) | b4
    }
}

fn main() {
    // Create a dummy TCP header as bytes
    let tcp_header_bytes: [u8; 20] = [
        0x00, 0x50, // Source Port: 80
        0x01, 0xBB, // Destination Port: 443
        0x12, 0x34, 0x56, 0x78, // Sequence Number
        0x9A, 0xBC, 0xDE, 0xF0, // Acknowledgement Number
        0x50, 0x02, // Data Offset, Reserved, Flags
        0xFF, 0xFF, // Window Size
        0x00, 0x00, // Checksum
        0x00, 0x00, // Urgent Pointer
    ];

    let header = TcpHeader::from_bytes(&tcp_header_bytes);
    println!("{:?}", header);

    let header_bytes = header.to_bytes();
    println!("{:?}", header_bytes);
}
