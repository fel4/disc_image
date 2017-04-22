#![allow(non_camel_case_types)]

pub enum Endianness {
    Little,
    Big,
    LittleBig
}

trait ISONumber {
    fn endianness(&self) -> Endianness;
}

macro_rules! number {
    ( $name:ident; $($sz:ty),+; $en:ident ) => (
        pub struct $name($($sz,)+);

        impl ISONumber for $name {
            fn endianness(&self) -> Endianness { $crate::numbers::Endianness::$en }
        } 
    )
}

number!(sint8; i8; Little);
number!(int8; u8; Little);
number!(sint16_LSB; i16; Little);
number!(sint16_MSB; i16; Big);
number!(sint16_LSBMSB; i16,i16; LittleBig);
number!(int16_LSB; u16; Little);
number!(int16_MSB; u16; Big);
number!(int16_LSBMSB; u16,u16; LittleBig);
number!(sint32_LSB; i32; Little);
number!(sint32_MSB; i32; Big);
number!(sint32_LSBMSB; i32,i32; LittleBig);
number!(int32_LSB; u32; Little);
number!(int32_MSB; u32; Big);
number!(int32_LSBMSB; u32,u32; LittleBig);

#[cfg(test)]
mod tests {
    use super::*;
}