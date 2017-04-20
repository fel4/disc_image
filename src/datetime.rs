use nom;
use super::strings::{self, FromISOString};

#[derive(Debug, PartialEq)]
pub struct DateTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    second_part: u8,
    tmz_offset: u8
}

impl DateTime {
    fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8, sp: u8, tmz: u8) -> DateTime {
        DateTime {
            year: year,
            month: month,
            day: day,
            hour: hour,
            minute: minute,
            second: second,
            second_part: sp,
            tmz_offset: tmz
        }
    }

    pub fn now() -> DateTime { unimplemented!(); }

    named!(pub parse<DateTime>,
        do_parse!(
            year: map_res!(call!(strings::dstring, 4), u16::from_isostring) >>
            month: map_res!(call!(strings::dstring, 2), u8::from_isostring) >>
            day: map_res!(call!(strings::dstring, 2), u8::from_isostring) >>
            hour: map_res!(call!(strings::dstring, 2), u8::from_isostring) >>
            minute: map_res!(call!(strings::dstring, 2), u8::from_isostring) >>
            second: map_res!(call!(strings::dstring, 2), u8::from_isostring) >>
            sp: map_res!(call!(strings::dstring, 2), u8::from_isostring) >>
            tmz: call!(nom::le_u8) >>
            (DateTime::new(year, month, day, hour, minute, second, sp, tmz))
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_datetime() {
        let data = &b"2017020405403400\x00"[..];
        let dtr = DateTime::parse(data);
        assert!(dtr.is_done());
        let dt = dtr.unwrap().1;
        println!("{:?}", dt);
        assert!(dt == DateTime::new(2017, 2, 4, 5, 40, 34, 00, 0));
    }
}