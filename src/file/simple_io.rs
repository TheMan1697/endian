use std::fs::File;
use std::io::{self, BufRead, Read, Result, Write};

fn read_from_input() -> Result<()> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);
    let mut writer = File::create("output.bin")?;

    for line in reader.lines() {
        let line = line?;
        let num: u16 = line.parse().unwrap();

        writer.write_all(&num.to_be_bytes())?;
    }
    Ok(())
}

fn read_from_output() -> Result<()> {
    let mut file = File::open("output.bin")?;
    let mut buffer = [0; 2];

    while let Ok(_) = file.read_exact(&mut buffer) {
        let num = u16::from_be_bytes(buffer);
        println!("{:?}", num);
    }
    Ok(())
}

fn main() -> Result<()> {
    read_from_input()?;
    read_from_output()?;
    Ok(())
}
