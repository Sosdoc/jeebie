use std::fs::File;
use std::io::Read;

use jeebie::utils::combine_as_u16;

/// A struct representing data contained in a gameboy cartridge (a.k.a. ROM).
/// Each Cartridge has an header with metadata (name, available hw on cart like rumble, ram, etc.)
/// Data contained in a cartridge is exposed as a Vec<u8> but should be accessed via the MMU.
#[derive(Debug)]
pub struct Cartridge {
    pub size: usize,
    pub name: String,
    pub data: Vec<u8>,
}

impl Cartridge {
    /// Creates a Cartridge by loading the file at the specified path.
    pub fn new_with_path(path: &str) -> Cartridge {
        Cartridge::new_with_vec(Cartridge::load_rom_file(path))
    }

    /// Creates a new Cartridge struct from a vector buffer.
    pub fn new_with_vec(data: Vec<u8>) -> Cartridge {
        let name_data : Vec<u8> = (&data[0x134..0x143]).to_vec();

        // read name data as ASCII string
        let name : String = name_data.into_iter()
            .map(| c | { c as char })
            .collect();

        let cgb_flag = data[0x143];

        let licensee_code_data : Vec<u8> = (&data[0x144..0x145]).to_vec();

        // read licensee code as ASCII string
        let name : String = licensee_code_data.into_iter()
            .map(| c | { c as char })
            .collect();

        let sgb_flag = data[0x146];

        let mbc_type = data[0x147];
        let rom_size = data[0x148];
        let ram_size = data[0x149];

        let dest_code = data[0x14A];
        let lic_code = data[0x14B];
        let version = data[0x14C];

        // only header_checksum is verified in real hardware
        let header_checksum = data[0x14D];
        let checksum = combine_as_u16(data[0x14E], data[0x14F]);

        // TODO: process the read data and save it in the cart struct
        Cartridge {
            size: data.len(),
            name: name,
            data: data,
        }
    }

    /// Loads binary data from a file into a vector buffer.
    fn load_rom_file(path: &str) -> Vec<u8> {
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
