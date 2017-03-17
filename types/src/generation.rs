use types::*;

use cookie_factory::*;

pub fn gen_value<'a>(x: (&'a mut [u8], usize), v: &AMQPValue) -> Result<(&'a mut [u8], usize), GenError> {
    match *v {
        AMQPValue::Boolean(ref b)        => do_gen!(x, gen_type(&v.get_type()) >> gen_boolean(b)),
        AMQPValue::ShortShortInt(ref i)  => do_gen!(x, gen_type(&v.get_type()) >> gen_short_short_int(i)),
        AMQPValue::ShortShortUInt(ref u) => do_gen!(x, gen_type(&v.get_type()) >> gen_short_short_uint(u)),
        AMQPValue::ShortInt(ref i)       => do_gen!(x, gen_type(&v.get_type()) >> gen_short_int(i)),
        AMQPValue::ShortUInt(ref u)      => do_gen!(x, gen_type(&v.get_type()) >> gen_short_uint(u)),
        AMQPValue::LongInt(ref i)        => do_gen!(x, gen_type(&v.get_type()) >> gen_long_int(i)),
        AMQPValue::LongUInt(ref u)       => do_gen!(x, gen_type(&v.get_type()) >> gen_long_uint(u)),
        AMQPValue::LongLongInt(ref i)    => do_gen!(x, gen_type(&v.get_type()) >> gen_long_long_int(i)),
        AMQPValue::LongLongUInt(ref u)   => do_gen!(x, gen_type(&v.get_type()) >> gen_long_long_uint(u)),
        AMQPValue::Float(ref f)          => do_gen!(x, gen_type(&v.get_type()) >> gen_float(f)),
        AMQPValue::Double(ref d)         => do_gen!(x, gen_type(&v.get_type()) >> gen_double(d)),
        AMQPValue::DecimalValue(ref d)   => do_gen!(x, gen_type(&v.get_type()) >> gen_decimal_value(d)),
        AMQPValue::ShortString(ref s)    => do_gen!(x, gen_type(&v.get_type()) >> gen_short_string(s)),
        AMQPValue::LongString(ref s)     => do_gen!(x, gen_type(&v.get_type()) >> gen_long_string(s)),
        AMQPValue::FieldArray(ref a)     => do_gen!(x, gen_type(&v.get_type()) >> gen_field_array(a)),
        AMQPValue::Timestamp(ref t)      => do_gen!(x, gen_type(&v.get_type()) >> gen_timestamp(t)),
        AMQPValue::FieldTable(ref t)     => do_gen!(x, gen_type(&v.get_type()) >> gen_field_table(t)),
        AMQPValue::Void                  => gen_type(x, &v.get_type()),
    }
}

pub fn gen_type<'a>(x: (&'a mut [u8], usize), t: &AMQPType) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_u8!(x, t.get_id() as u8)
}

pub fn gen_boolean<'a>(x: (&'a mut [u8], usize), b: &Boolean) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_u8!(x, if *b { 1 } else { 0 })
}

pub fn gen_short_short_int<'a>(x: (&'a mut [u8], usize), i: &ShortShortInt) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_i8!(x, *i)
}

pub fn gen_short_short_uint<'a>(x: (&'a mut [u8], usize), u: &ShortShortUInt) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_u8!(x, *u)
}

pub fn gen_short_int<'a>(x: (&'a mut [u8], usize), i: &ShortInt) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_i16!(x, *i)
}

pub fn gen_short_uint<'a>(x: (&'a mut [u8], usize), u: &ShortUInt) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_u16!(x, *u)
}

pub fn gen_long_int<'a>(x: (&'a mut [u8], usize), i: &LongInt) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_i32!(x, *i)
}

pub fn gen_long_uint<'a>(x: (&'a mut [u8], usize), u: &LongUInt) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_u32!(x, *u)
}

pub fn gen_long_long_int<'a>(x: (&'a mut [u8], usize), i: &LongLongInt) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_i64!(x, *i)
}

pub fn gen_long_long_uint<'a>(x: (&'a mut [u8], usize), u: &LongLongUInt) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_u64!(x, *u)
}

pub fn gen_float<'a>(x: (&'a mut [u8], usize), f: &Float) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_f32!(x, *f)
}

pub fn gen_double<'a>(x: (&'a mut [u8], usize), d: &Double) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_f64!(x, *d)
}

pub fn gen_decimal_value<'a>(x: (&'a mut [u8], usize), d: &DecimalValue) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x, gen_short_short_uint(&d.scale) >> gen_long_uint(&d.value))
}

pub fn gen_short_string<'a>(x: (&'a mut [u8], usize), s: &ShortString) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x, gen_short_short_uint(&(s.len() as ShortShortUInt)) >> gen_slice!(s.as_bytes()))
}

pub fn gen_long_string<'a>(x: (&'a mut [u8], usize), s: &LongString) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x, gen_long_uint(&(s.len() as LongUInt)) >> gen_slice!(s.as_bytes()))
}

pub fn gen_field_array<'a>(x: (&'a mut [u8], usize), a: &FieldArray) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x, gen_long_uint(&(a.len() as LongUInt)) >> gen_many_ref!(a, gen_value))
}

pub fn gen_timestamp<'a>(x: (&'a mut [u8], usize), t: &Timestamp) -> Result<(&'a mut [u8], usize), GenError> {
    gen_long_long_uint(x, t)
}

pub fn gen_field_table<'a>(x: (&'a mut [u8], usize), t: &FieldTable) -> Result<(&'a mut [u8], usize), GenError> {
    let (x1, index1) = x;
    gen_many_ref!((x1, index1 + 4), t, gen_field_entry).and_then(|(x2, index2)| {
        gen_long_uint((x2, index1), &((index2 - index1 - 4) as LongUInt)).and_then(|(x3, _)| Ok((x3, index2)))
    })
}

fn gen_field_entry<'a>(x: (&'a mut [u8], usize), e: &(&ShortString, &AMQPValue)) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x, gen_short_string(e.0) >> gen_value(e.1))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gen_type() {
        assert_eq!(gen_type((&mut [0], 0), &AMQPType::ShortShortInt).unwrap(), (&mut [98][..], 1));
        assert_eq!(gen_type((&mut [0], 0), &AMQPType::ShortInt).unwrap(),      (&mut [85][..], 1));
    }

    #[test]
    fn test_gen_boolean() {
        assert_eq!(gen_boolean((&mut [0], 0), &false).unwrap(), (&mut [0][..], 1));
        assert_eq!(gen_boolean((&mut [0], 0), &true).unwrap(),  (&mut [1][..], 1));
    }

    #[test]
    fn test_gen_short_short_int() {
        assert_eq!(gen_short_short_int((&mut [0], 0), &0).unwrap(),  (&mut [0][..],   1));
        assert_eq!(gen_short_short_int((&mut [0], 0), &-1).unwrap(), (&mut [255][..], 1));
    }

    #[test]
    fn test_gen_short_short_uint() {
        assert_eq!(gen_short_short_uint((&mut [0], 0), &0).unwrap(),   (&mut [0][..],   1));
        assert_eq!(gen_short_short_uint((&mut [0], 0), &255).unwrap(), (&mut [255][..], 1));
    }

    #[test]
    fn test_gen_short_int() {
        assert_eq!(gen_short_int((&mut [0, 0], 0), &0).unwrap(),  (&mut [0,   0][..],   2));
        assert_eq!(gen_short_int((&mut [0, 0], 0), &-1).unwrap(), (&mut [255, 255][..], 2));
    }

    #[test]
    fn test_gen_short_uint() {
        assert_eq!(gen_short_uint((&mut [0, 0], 0), &0).unwrap(),     (&mut [0,   0][..],   2));
        assert_eq!(gen_short_uint((&mut [0, 0], 0), &65535).unwrap(), (&mut [255, 255][..], 2));
    }

    #[test]
    fn test_gen_long_int() {
        assert_eq!(gen_long_int((&mut [0, 0, 0, 0], 0), &0).unwrap(),  (&mut [0,   0,   0,   0][..],   4));
        assert_eq!(gen_long_int((&mut [0, 0, 0, 0], 0), &-1).unwrap(), (&mut [255, 255, 255, 255][..], 4));
    }

    #[test]
    fn test_gen_long_uint() {
        assert_eq!(gen_long_uint((&mut [0, 0, 0, 0], 0), &0).unwrap(),          (&mut [0,   0,   0,   0][..],   4));
        assert_eq!(gen_long_uint((&mut [0, 0, 0, 0], 0), &4294967295).unwrap(), (&mut [255, 255, 255, 255][..], 4));
    }

    #[test]
    fn test_gen_long_long_int() {
        assert_eq!(gen_long_long_int((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), &0).unwrap(),  (&mut [0,   0,   0,   0,   0,   0,   0,   0][..],   8));
        assert_eq!(gen_long_long_int((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), &-1).unwrap(), (&mut [255, 255, 255, 255, 255, 255, 255, 255][..], 8));
    }

    #[test]
    fn test_gen_long_long_uint() {
        assert_eq!(gen_long_long_uint((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), &0).unwrap(),                    (&mut [0,   0,   0,   0,   0,   0,   0,   0][..],   8));
        assert_eq!(gen_long_long_uint((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), &18446744073709551615).unwrap(), (&mut [255, 255, 255, 255, 255, 255, 255, 255][..], 8));
    }

    #[test]
    fn test_gen_float() {
        assert_eq!(gen_float((&mut [0, 0, 0, 0], 0), &0.).unwrap(),    (&mut [0,  0,  0,   0][..],  4));
        assert_eq!(gen_float((&mut [0, 0, 0, 0], 0), &42.42).unwrap(), (&mut [66, 41, 174, 20][..], 4));
    }

    #[test]
    fn test_gen_double() {
        assert_eq!(gen_double((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), &0.).unwrap(),    (&mut [0,  0,  0,  0,   0,   0,  0,  0][..],   8));
        assert_eq!(gen_double((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), &42.42).unwrap(), (&mut [64, 69, 53, 194, 143, 92, 40, 246][..], 8));
    }

    #[test]
    fn test_gen_decimal_value() {
        assert_eq!(gen_decimal_value((&mut [0, 0, 0, 0, 0], 0), &DecimalValue { scale: 0, value: 0 }).unwrap(),  (&mut [0, 0, 0, 0, 0][..], 5));
        assert_eq!(gen_decimal_value((&mut [0, 0, 0, 0, 0], 0), &DecimalValue { scale: 2, value: 42 }).unwrap(), (&mut [2, 0, 0, 0, 42][..], 5));
    }

    /* The string tests don't pass because of a bug in cookie factory
    #[test]
    fn test_gen_short_string() {
        assert_eq!(gen_short_string((&mut [0], 0), &"".to_string()).unwrap(),                 (&mut [0][..], 1));
        assert_eq!(gen_short_string((&mut [0, 0, 0, 0, 0], 0), &"test".to_string()).unwrap(), (&mut [4, 116, 101, 115, 116][..], 5));
    }

    #[test]
    fn test_gen_long_string() {
        assert_eq!(gen_long_string((&mut [0, 0, 0, 0], 0), &"".to_string()).unwrap(),                 (&mut [0, 0, 0, 0][..], 4));
        assert_eq!(gen_long_string((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), &"test".to_string()).unwrap(), (&mut [0, 0, 0, 4, 116, 101, 115, 116][..], 8));
    }
    */

    #[test]
    fn test_gen_field_array() {
        assert_eq!(gen_field_array((&mut [0, 0, 0, 0], 0), &FieldArray::new()).unwrap(),                    (&mut [0, 0, 0, 0][..], 4));
        assert_eq!(gen_field_array((&mut [0, 0, 0, 0, 0, 0], 0), &vec![AMQPValue::Boolean(true)]).unwrap(), (&mut [0, 0, 0, 1, 116, 1][..], 6));
    }

    #[test]
    fn test_gen_timestamp() {
        assert_eq!(gen_timestamp((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), &0).unwrap(),                    (&mut [0,   0,   0,   0,   0,   0,   0,   0][..],   8));
        assert_eq!(gen_timestamp((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), &18446744073709551615).unwrap(), (&mut [255, 255, 255, 255, 255, 255, 255, 255][..], 8));
    }

    #[test]
    fn test_gen_field_table() {
        let mut table = FieldTable::new();
        table.insert("test".to_string(), AMQPValue::Float(42.42));
        table.insert("tt".to_string(),   AMQPValue::LongLongInt(42));
        assert_eq!(gen_field_table((&mut [0, 0, 0, 0],                                                                   0), &FieldTable::new()).unwrap(), (&mut [0, 0, 0, 0][..],                                                                                         4));
        assert_eq!(gen_field_table((&mut [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0), &table).unwrap(),             (&mut [0, 0, 0, 22, 4, 116, 101, 115, 116, 102, 66, 41, 174, 20, 2, 116, 116, 76, 0, 0, 0, 0, 0, 0, 0, 42][..], 26));
    }
}