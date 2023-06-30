use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Read};

#[derive(Debug)]
struct IPv4Header {
    version: u8,
    ihl: u8,
    dscp: u8,
    ecn: u8,
    total_length: u16,
    identification: u16,
    flags: u16,
    fragment_offset: u16,
    ttl: u8,
    protocol: u8,
    header_checksum: u16,
    source_ip: [u8; 4],
    dest_ip: [u8; 4],
}

impl IPv4Header {
    fn new() -> IPv4Header {
        IPv4Header {
            version: 4,
            ihl: 5,
            dscp: 0,
            ecn: 0,
            total_length: 20,
            identification: 0,
            flags: 2,
            fragment_offset: 0,
            ttl: 64,
            protocol: 1,
            header_checksum: 0,
            source_ip: [192, 168, 1, 1],
            dest_ip: [192, 168, 1, 2],
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.write_u8((self.version << 4) | self.ihl).unwrap();
        bytes.write_u8((self.dscp << 2) | self.ecn).unwrap();
        bytes.write_u16::<BigEndian>(self.total_length).unwrap();
        bytes.write_u16::<BigEndian>(self.identification).unwrap();
        bytes
            .write_u16::<BigEndian>((self.flags << 13) | self.fragment_offset)
            .unwrap();
        bytes.write_u8(self.ttl).unwrap();
        bytes.write_u8(self.protocol).unwrap();
        bytes.write_u16::<BigEndian>(self.header_checksum).unwrap();
        bytes.extend_from_slice(&self.source_ip);
        bytes.extend_from_slice(&self.dest_ip);
        bytes
    }

    fn from_bytes(bytes: Vec<u8>) -> IPv4Header {
        let mut rdr = Cursor::new(bytes);
        let version_ihl = rdr.read_u8().unwrap();
        let version = version_ihl >> 4;
        let ihl = version_ihl & 0xf;
        let dscp_ecn = rdr.read_u8().unwrap();
        let dscp = dscp_ecn >> 2;
        let ecn = dscp_ecn & 0x3;
        let total_length = rdr.read_u16::<BigEndian>().unwrap();
        let identification = rdr.read_u16::<BigEndian>().unwrap();
        let flags_fragment_offset = rdr.read_u16::<BigEndian>().unwrap();
        let flags = (flags_fragment_offset >> 13) & 0x7;
        let fragment_offset = flags_fragment_offset & 0x1fff;
        let ttl = rdr.read_u8().unwrap();
        let protocol = rdr.read_u8().unwrap();
        let header_checksum = rdr.read_u16::<BigEndian>().unwrap();
        let mut source_ip = [0u8; 4];
        rdr.read_exact(&mut source_ip).unwrap();
        let mut dest_ip = [0u8; 4];
        rdr.read_exact(&mut dest_ip).unwrap();

        IPv4Header {
            version,
            ihl,
            dscp,
            ecn,
            total_length,
            identification,
            flags,
            fragment_offset,
            ttl,
            protocol,
            header_checksum,
            source_ip,
            dest_ip,
        }
    }
}

fn main() {
    let header = IPv4Header::new();
    println!("Original Header: {:?}", header);

    let bytes = header.to_bytes();
    println!("Header as Bytes: {:?}", bytes);

    let restored_header = IPv4Header::from_bytes(bytes);
    println!("Restored Header: {:?}", restored_header)
}
