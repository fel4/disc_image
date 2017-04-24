#![allow(non_snake_case)]

use nom;

named!(pub sint8<i8>, call!(nom::le_i8));
named!(pub int8<u8>, call!(nom::le_u8));
named!(pub sint16_LSB<i16>, i16!(nom::Endianness::Little));
named!(pub sint16_MSB<i16>, i16!(nom::Endianness::Big));
named!(pub sint16_LSBMSB<(i16,i16)>,
    do_parse!(
        x: call!(sint16_LSB) >>
        y: call!(sint16_MSB) >>
        (x,y)
    )
);
named!(pub int16_LSB<u16>, u16!(nom::Endianness::Little));
named!(pub int16_MSB<u16>, u16!(nom::Endianness::Big));
named!(pub int16_LSBMSB<(u16, u16)>,
    do_parse!(
        x: call!(int16_LSB) >>
        y: call!(int16_MSB) >>
        (x,y)
    )
);
named!(pub sint32_LSB<i32>, i32!(nom::Endianness::Little));
named!(pub sint32_MSB<i32>, i32!(nom::Endianness::Big));
named!(pub sint32_LSBMSB<(i32,i32)>,
    do_parse!(
        x: call!(sint32_LSB) >>
        y: call!(sint32_MSB) >>
        (x,y)
    )
);
named!(pub int32_LSB<u32>, u32!(nom::Endianness::Little));
named!(pub int32_MSB<u32>, u32!(nom::Endianness::Big));
named!(pub int32_LSBMSB<(u32, u32)>,
    do_parse!(
        x: call!(int32_LSB) >>
        y: call!(int32_MSB) >>
        (x,y)
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sint8_valid() { unimplemented!(); }

    #[test]
    fn sint8_invalid() { unimplemented!(); }

    #[test]
    fn sint16LSB_valid() { unimplemented!(); }

    #[test]
    fn sint16LSB_invalid() { unimplemented!(); }

    #[test]
    fn sint16MSB_valid() { unimplemented!(); }

    #[test]
    fn sint16MSB_invalid() { unimplemented!(); }

    #[test]
    fn sint16LSBMSB_valid() { unimplemented!(); }

    #[test]
    fn sint16LSBMSB_invalid() { unimplemented!(); }

    #[test]
    fn sint32LSB_valid() { unimplemented!(); }

    #[test]
    fn sint32LSB_invalid() { unimplemented!(); }

    #[test]
    fn sint32MSB_valid() { unimplemented!(); }

    #[test]
    fn sint32MSB_invalid() { unimplemented!(); }

    #[test]
    fn sint32LSBMSB_valid() { unimplemented!(); }

    #[test]
    fn sint32LSBMSB_invalid() { unimplemented!(); }

    #[test]
    fn int8_valid() { unimplemented!(); }

    #[test]
    fn int8_invalid() { unimplemented!(); }

    #[test]
    fn int16LSB_valid() { unimplemented!(); }

    #[test]
    fn int16LSB_invalid() { unimplemented!(); }

    #[test]
    fn int16MSB_valid() { unimplemented!(); }

    #[test]
    fn int16MSB_invalid() { unimplemented!(); }

    #[test]
    fn int16LSBMSB_valid() { unimplemented!(); }

    #[test]
    fn int16LSBMSB_invalid() { unimplemented!(); }

    #[test]
    fn int32LSB_valid() { unimplemented!(); }

    #[test]
    fn int32LSB_invalid() { unimplemented!(); }

    #[test]
    fn int32MSB_valid() { unimplemented!(); }

    #[test]
    fn int32MSB_invalid() { unimplemented!(); }

    #[test]
    fn int32LSBMSB_valid() { unimplemented!(); }

    #[test]
    fn int32LSBMSB_invalid() { unimplemented!(); }
}