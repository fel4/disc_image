#![allow(dead_code)]

#[macro_use]
extern crate nom;

mod datetime;
mod strings;

pub const SECTOR_SIZE: usize = 2048;

#[allow(non_camel_case_types)]
enum NumberFormat {
    int8(u8),
    sint8(i8),
    int16_LSB(u16),
    int16_MSB(u16),
    int16_LSBMSB(u16, u16),
    sint16_LSB(i16),
    sint16_MSB(i16),
    sint16_LSBMSV(i16,i16),
    int32_LSB(u32),
    int32_MSB(u32),
    int32_LSBMSB(u32,u32),
    sint32_LSB(i32),
    sint32_MSB(i32),
    sint32_LSBMSB(i32,i32)
}

struct Sector(pub [u8; SECTOR_SIZE]);


pub struct ImageFile {

}


named!(take_sector, take!(SECTOR_SIZE));


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    use super::*;

    #[test]
    #[ignore]
    fn test_file_sanity_check() {
        let mut buffer: Vec<u8> = Vec::new();
        let mut file = File::open("../../shard.iso").unwrap();
        let len = file.read_to_end(&mut buffer).unwrap();
        let sector_sz: usize = SECTOR_SIZE as usize;
        let sector_count = len / sector_sz;
        println!("file size: {}", len);
        println!("number of sectors: {}", sector_count);
        assert_eq!(len, sector_count * sector_sz);
    }
}
