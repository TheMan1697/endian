use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Read};

#[derive(Debug)]
struct TcpHeader {
    source_port: u16,
    destination_port: u16,
    sequence_number: u32,
    acknowledgement_number: u32,
    data_offset: u8,
    reserved: u8,
    flags: u8,
    window_size: u16,
    checksum: u16,
    urgent_pointer: u16,
    options: Vec<u8>,
}

impl TcpHeader {
    fn from_bytes(bytes: &[u8]) -> TcpHeader {
        let (header_bytes, options) = bytes.split_at(20);

        let mut iter = header_bytes.chunks_exact(2);
        let source_port = u16::from_be_bytes(iter.next().unwrap().try_into().unwrap());
        let destination_port = u16::from_be_bytes(iter.next().unwrap().try_into().unwrap());

        let sequence_number = u32::from_be_bytes(iter.next().unwrap().try_into().unwrap());
        let acknowledgement_number = u32::from_be_bytes(iter.next().unwrap().try_into().unwrap());

        let mut iter = iter.next().unwrap().chunks_exact(1);
        let data_offset = iter.next().unwrap()[0] >> 4;
        let reserved = (iter.next().unwrap()[0] & 0x0F) >> 1;
        let flags = (iter.next().unwrap()[0] & 0x01) << 7 | (iter.next().unwrap()[0] >> 1);

        let window_size = u16::from_be_bytes(iter.next().unwrap().try_into().unwrap());
        let checksum = u16::from_be_bytes(iter.next().unwrap().try_into().unwrap());
        let urgent_pointer = u16::from_be_bytes(iter.next().unwrap().try_into().unwrap());

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
            options: options.to_vec(),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(&self.source_port.to_be_bytes());
        bytes.extend(&self.destination_port.to_be_bytes());
        bytes.extend(&self.sequence_number.to_be_bytes());
        bytes.extend(&self.acknowledgement_number.to_be_bytes());
        let data_offset_reserved_flags =
            (self.data_offset << 4) | (self.reserved << 1) | (self.flags >> 7);
        bytes.push(data_offset_reserved_flags);
        let flags = (self.flags << 1) & 0xFE;
        bytes.push(flags);
        bytes.extend(&self.window_size.to_be_bytes());
        bytes.extend(&self.checksum.to_be_bytes());
        bytes.extend(&self.urgent_pointer.to_be_bytes());
        bytes.extend(&self.options);

        bytes
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

    // Parse bytes into a TcpHeader
    let header = TcpHeader::from_bytes(&tcp_header_bytes);
    println!("{:?}", header);

    // Convert TcpHeader back into bytes
    let header_bytes = header.to_bytes();
    println!("{:?}", header_bytes);
}

