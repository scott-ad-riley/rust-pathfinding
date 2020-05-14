pub fn integer_decode(val: f64) -> (u64, i16, i8) {
    let bits: u64 = val.to_bits();
    let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0x_fffffffffffff) << 1
    } else {
        (bits & 0x_fffffffffffff) | 0x_10000000000000
    };

    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
}
