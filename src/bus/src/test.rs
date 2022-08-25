use super::*;
use std::io::Write;
use std::os::unix::prelude::*;
use tempfile::NamedTempFile;
use filename::file_name;
use crate::inesformat::format::{CHR_ROM_SIZE_FACTOR, PRG_ROM_SIZE_FACTOR};

pub fn generate_rom(add_trainer: bool, mapper_id: u8, ines_file_version: u8) -> (NamedTempFile, String) {
    let mut tmp_file = NamedTempFile::new().unwrap();

    // header
    let mut contents:Vec<u8> = vec![
        0x4E,
        0x45,
        0x53,
        0x1A,
        1,
        1,
        (mapper_id & 0x0F) << 4, // flag6
        (mapper_id & 0xF0),      // flag7
        0, 0, 0, 0, 0, 0, 0, 0];

    if add_trainer {
        contents[6] |= 0x4;
        contents.resize(contents.len() + 512, 0xFF);
    }

    if ines_file_version == 1 {
        //  prg_rom
        contents.resize(contents.len() + contents[4] as usize * PRG_ROM_SIZE_FACTOR, 0xEE);

        //  chr_rom
        contents.resize(contents.len() + contents[5] as usize * CHR_ROM_SIZE_FACTOR, 0xDD);
    } else if ines_file_version == 2 {
        contents[7] |= 0x08;
    }

    tmp_file.write_all(contents.as_slice()).expect("failed to write");

    // filename
    let filename = file_name(&tmp_file.as_raw_fd()).unwrap();
    let os_str = filename.into_os_string();

    (tmp_file, String::from(os_str.to_str().unwrap()))
}

#[test]
fn test_memory_is_zeroed() {
    let bus = Bus::new();
    assert_eq!(&[0; RAM_SIZE as usize + 1], &bus.cpu_ram[..]);
}
