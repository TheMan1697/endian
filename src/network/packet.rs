use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

#[derive(Debug)]
struct Packet {
    src_port: u16,
    dst_port: u16,
    length: u16,
    checksum: u16,
}

impl Packet {
    fn new(src_port: u16, dst_port: u16, length: u16, checksum: u16) -> Packet {
        Packet {
            src_port,
            dst_port,
            length,
            checksum,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.write_u16::<BigEndian>(self.src_port).unwrap();
        bytes.write_u16::<BigEndian>(self.dst_port).unwrap();
        bytes.write_u16::<BigEndian>(self.length).unwrap();
        bytes.write_u16::<BigEndian>(self.checksum).unwrap();
        bytes
    }

    fn from_bytes(bytes: Vec<u8>) -> Packet {
        let mut rdr = Cursor::new(bytes);
        let src_port = rdr.read_u16::<BigEndian>().unwrap();
        let dst_port = rdr.read_u16::<BigEndian>().unwrap();
        let length = rdr.read_u16::<BigEndian>().unwrap();
        let checksum = rdr.read_u16::<BigEndian>().unwrap();

        Packet {
            src_port,
            dst_port,
            length,
            checksum,
        }
    }
}

fn main() {
    let packet = Packet::new(12345, 6789, 1024, 5678);
    println!("Origianl Packet: {:?}", packet);

    let bytes = packet.to_bytes();
    println!("Packet as Bytes: {:?}", bytes);

    let restored_packet = Packet::from_bytes(bytes);
    println!("Restored Pactet: {:?}", restored_packet);
}
