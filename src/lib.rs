use std::fs::File;
use std::io;
use std::io::*;
use std::vec::Vec;

pub fn cat(path: String) -> Result<()> {
    let mut f = File::open(path).expect("File not found");
    let mut buf: Vec<u8> = Vec::new();

    f.read_to_end(&mut buf)?;
    io::stdout().write_all(buf.as_slice())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
