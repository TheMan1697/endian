use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

#[derive(Debug)]
struct SimpleStruct {
    field1: u32,
    field2: u16,
    field3: u8
}

fn main() {
    let mut data = vec![];


    let simple_struct = SimpleStruct {
        field1: 12345,
        field2: 6789,
        field3: 101,
    };

    data.write_u32::<LittleEndian>(simple_struct.field1).unwrap();
    data.write_u16::<LittleEndian>(simple_struct.field2).unwrap();
    data.write_u8(simple_struct.field3).unwrap();

    let mut cursor = Cursor::new(data);
    let mut data_be = vec![];
    let field1_le = cursor.read_u32::<LittleEndian>().unwrap();
    let field2_le = cursor.read_u16::<LittleEndian>().unwrap();
    let field3_le = cursor.read_u8().unwrap();

    data_be.write_u32::<BigEndian>(field1_le).unwrap();
    data_be.write_u16::<BigEndian>(field2_le).unwrap();
    data_be.write_u8(field3_le).unwrap();

    let mut cursor = Cursor::new(&data_be);

    let simple_struct_be = SimpleStruct {
        field1: cursor.read_u32::<BigEndian>().unwrap(),
        field2: cursor.read_u16::<BigEndian>().unwrap(),
        field3: cursor.read_u8().unwrap(),
    };

    println!("리틀 엔디언 {:?}", simple_struct);
    println!("빅 엔디언 {:?}", simple_struct_be);
}