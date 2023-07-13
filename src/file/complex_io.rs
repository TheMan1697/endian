use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fs::File;
use std::io::{BufWriter, Result};

#[derive(Debug)]
struct MyStruct {
    a: i32,
    b: f64,
    c: u16,
}

impl MyStruct {
    fn write_to_file(&self, writer: &mut BufWriter<File>) -> Result<()> {
        writer.write_i32::<LittleEndian>(self.a)?;
        writer.write_f64::<LittleEndian>(self.b)?;
        writer.write_u16::<LittleEndian>(self.c)?;
        Ok(())
    }

    fn read_from_file(reader: &mut File) -> Result<Self> {
        let a = reader.read_i32::<LittleEndian>()?;
        let b = reader.read_f64::<LittleEndian>()?;
        let c = reader.read_u16::<LittleEndian>()?;
        Ok(MyStruct { a, b, c })
    }
}

fn main() -> Result<()> {
    let data = vec![
        MyStruct {
            a: 42,
            b: 3.14159,
            c: 255,
        },
        MyStruct {
            a: 53,
            b: 2.71828,
            c: 512,
        },
        MyStruct {
            a: 64,
            b: 1.41421,
            c: 1024,
        },
    ];

    {
        let file = File::create("data.bin")?;
        let mut writer = BufWriter::new(file);

        for struct_instance in &data {
            struct_instance.write_to_file(&mut writer)?;
        }
    }

    {
        let file = File::open("data.bin")?;
        let mut reader = file;

        for _ in 0..data.len() {
            let loaded_data = MyStruct::read_from_file(&mut reader)?;
            println!("{:?}", loaded_data);
        }
    }

    Ok(())
}
