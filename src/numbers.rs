#![allow(non_snake_case)]

use nom;

named!(pub sint8<i8>, call!(nom::le_i8));
named!(pub int8<u8>, call!(nom::le_u8));
named!(pub sint16_LSB<i16>, i16!(nom::Endianness::Little));
named!(pub sint16_MSB<i16>, i16!(nom::Endianness::Big));
named!(pub sint16_LSBMSB<(i16,i16)>,
    verify!(
        do_parse!(
            x: call!(sint16_LSB) >>
            y: call!(sint16_MSB) >>
            (x,y)
        ),
        |val: (i16,i16)| val.0 == val.1
    )
);
named!(pub int16_LSB<u16>, u16!(nom::Endianness::Little));
named!(pub int16_MSB<u16>, u16!(nom::Endianness::Big));
named!(pub int16_LSBMSB<(u16, u16)>,
    verify!(
        do_parse!(
            x: call!(int16_LSB) >>
            y: call!(int16_MSB) >>
            (x,y)
        ),
        |val: (u16,u16)| val.0 == val.1
    )
);
named!(pub sint32_LSB<i32>, i32!(nom::Endianness::Little));
named!(pub sint32_MSB<i32>, i32!(nom::Endianness::Big));
named!(pub sint32_LSBMSB<(i32,i32)>,
    verify!(
        do_parse!(
            x: call!(sint32_LSB) >>
            y: call!(sint32_MSB) >>
            (x,y)
        ),
        |val: (i32,i32)| val.0 == val.1
    )
);
named!(pub int32_LSB<u32>, u32!(nom::Endianness::Little));
named!(pub int32_MSB<u32>, u32!(nom::Endianness::Big));
named!(pub int32_LSBMSB<(u32, u32)>,
    verify!(
        do_parse!(
            x: call!(int32_LSB) >>
            y: call!(int32_MSB) >>
            (x,y)
        ),
        |val: (u32, u32)| val.0 == val.1
    )
);

#[cfg(test)]
mod tests {
    use nom::IResult;
    use super::*;

    #[test]
    fn sint8_valid() {
        let i = &b"\xFF"[..];
        assert_eq!(sint8(i), IResult::Done(&b""[..], -1));
    }

    #[test]
    fn sint8_invalid() {
        let i = &b"\xFF"[..];
        assert_ne!(sint8(i), IResult::Done(&b""[..], -2));
    }

    #[test]
    fn sint16LSB_valid() {
        let i = &b"\xFF\x00"[..];
        assert_eq!(sint16_LSB(i), IResult::Done(&b""[..], 255));
    }

    #[test]
    fn sint16LSB_invalid() {
        let i = &b"\x00\xFF"[..];
        assert_ne!(sint16_LSB(i), IResult::Done(&b""[..], 255));
    }

    #[test]
    fn sint16MSB_valid() {
        let i = &b"\x80\xFF"[..];
        assert_eq!(sint16_MSB(i), IResult::Done(&b""[..], -32513));
    }

    #[test]
    fn sint16MSB_invalid() {
        let i = &b"\xFF\x00"[..];
        assert_ne!(sint16_MSB(i), IResult::Done(&b""[..], 255));
    }

    #[test]
    fn sint16LSBMSB_valid() {
        let i = &b"\x00\xFF\xFF\x00"[..];
        assert_eq!(sint16_LSBMSB(i), IResult::Done(&b""[..], (-256, -256)));
        let i = &b"\xFF\x00\x00\xFF"[..];
        assert_eq!(sint16_LSBMSB(i), IResult::Done(&b""[..], (255, 255)));
    }

    #[test]
    fn sint16LSBMSB_invalid() {
        let i = &b"\x00\xFF\x00\xFF"[..];
        assert_ne!(sint16_LSBMSB(i), IResult::Done(&b""[..], (-256, -256)));
        let i = &b"\xFF\x00\xFF\x00"[..];
        assert_ne!(sint16_LSBMSB(i), IResult::Done(&b""[..], (255, 255)));
    }

    #[test]
    fn sint32LSB_valid() {
        let i = &b"\xFF\xFF\xFF\xFF"[..];
        assert_eq!(sint32_LSB(i), IResult::Done(&b""[..], -1));
        let i = &b"\xFF\x00\x00\x00"[..];
        assert_eq!(sint32_LSB(i), IResult::Done(&b""[..], 255));
     }

    #[test]
    fn sint32LSB_invalid() { unimplemented!(); }

    #[test]
    fn sint32MSB_valid() {
        let i = &b"\xFF\xFF\xFF\xFF"[..];
        assert_eq!(sint32_LSB(i), IResult::Done(&b""[..], -1));
        let i = &b"\x00\x00\x00\xFF"[..];
        assert_eq!(sint32_LSB(i), IResult::Done(&b""[..], 255));
     }

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