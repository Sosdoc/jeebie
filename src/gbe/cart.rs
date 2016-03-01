use std::fs::File;
use std::io::Read;

/// A struct representing data contained in a gameboy cartridge (a.k.a. ROM).
/// Each Cartridge has an header with metadata (name, available hw on cart like rumble, ram, etc.)
/// Data contained in a cartridge is exposed as a Vec<u8> but should be accessed via the MMU.
#[derive(Debug)]
pub struct Cartridge {
    size: usize,
    data: Vec<u8>,
}

impl Cartridge {
    /// Creates a Cartridge by loading the file at the specified path.
    pub fn new_with_path(path: &str) -> Cartridge {
        Cartridge::new_with_vec(Cartridge::load_rom_file(path))
    }

    /// Creates a new Cartridge struct from a vector buffer.
    pub fn new_with_vec(data: Vec<u8>) -> Cartridge {
        Cartridge {
            size: data.len(),
            data: data,
        }
    }

    /// Loads binary data from a file into a vector buffer.
    pub fn load_rom_file(path: &str) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];

        if let Ok(mut file) = File::open(path) {
            match file.read_to_end(&mut buf) {
                Ok(_) => {},
                Err(e) => println!("{:?}", e),
            }
        }
        buf
    }
}
