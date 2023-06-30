use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

#[derive(Debug)]
struct ComplexStruct {
    field1: u32,
    field2: u16,
    field3: u8,
    field4: Vec<u32>,
}

impl ComplexStruct {
    fn new() -> Self {
        ComplexStruct {
            field1: 12345,
            field2: 6789,
            field3: 101,
            field4: vec![1, 2, 3, 4, 5],
        }
    }

    fn to_big_endian(&self) -> Vec<u8> {
        let mut data_be = vec![];

        data_be.write_u32::<BigEndian>(self.field1).unwrap();
        data_be.write_u16::<BigEndian>(self.field2).unwrap();
        data_be.write_u8(self.field3).unwrap();
        for &item in &self.field4 {
            data_be.write_u32::<BigEndian>(item).unwrap();
        }
        data_be
    }

    fn from_big_endian(data: &[u8]) -> Self {
        let mut cursor = Cursor::new(data);
        let field1_be = cursor.read_u32::<BigEndian>().unwrap();
        let field2_be = cursor.read_u16::<BigEndian>().unwrap();
        let field3_be = cursor.read_u8().unwrap();

        let mut field4_be = vec![];

        while cursor.position() < data.len() as u64 {
            field4_be.push(cursor.read_u32::<BigEndian>().unwrap());
        }

        ComplexStruct {
            field1: field1_be,
            field2: field2_be,
            field3: field3_be,
            field4: field4_be,
        }
    }
}

fn main() {
    let complex_struct = ComplexStruct::new();
    let data_be = complex_struct.to_big_endian();

    println!("Original data: {:?}", complex_struct);
    println!("BigEndian data: {:?}", data_be);

    let complex_struct_be = ComplexStruct::from_big_endian(&data_be);

    println!("Data from BigEndian: {:?}", complex_struct_be)
}