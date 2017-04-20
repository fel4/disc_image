#[allow(non_camel_case_types)]
enum NumberFormat {
    int8(u8),
    sint8(i8),
    int16_LSB(u16),
    int16_MSB(u16),
    int16_LSBMSB(u16, u16),
    sint16_LSB(i16),
    sint16_MSB(i16),
    sint16_LSBMSB(i16,i16),
    int32_LSB(u32),
    int32_MSB(u32),
    int32_LSBMSB(u32,u32),
    sint32_LSB(i32),
    sint32_MSB(i32),
    sint32_LSBMSB(i32,i32)
}

pub enum Endianness {
    Little,
    Big,
    LittleBig
}

pub trait ISONumber {

}