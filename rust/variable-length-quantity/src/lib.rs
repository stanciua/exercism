// Variable Length Quantity
// Implement variable length quantity encoding and decoding.
//
// The goal of this exercise is to implement VLQ encoding/decoding.
//
// In short, the goal of this encoding is to encode integer values in a way that would save bytes. Only the first 7 bits of each byte is significant (right-justified; sort of like an ASCII byte). So, if you have a 32-bit value, you have to unpack it into a series of 7-bit bytes. Of course, you will have a variable number of bytes depending upon your integer. To indicate which is the last byte of the series, you leave bit #7 clear. In all of the preceding bytes, you set bit #7.
//
// So, if an integer is between 0-127, it can be represented as one byte. The largest integer allowed is 0FFFFFFF, which translates to 4 bytes variable length. Here are examples of delta-times as 32-bit values, and the variable length quantities that they translate to:
//
// 1
// 2
// 3
// 4
// 5
// 6
// 7
// 8
// 9
// 10
// 11
// 12
// 13
//  NUMBER        VARIABLE QUANTITY
// 00000000              00
// 00000040              40
// 0000007F              7F
// 00000080             81 00
// 00002000             C0 00
// 00003FFF             FF 7F
// 00004000           81 80 00
// 00100000           C0 80 00
// 001FFFFF           FF FF 7F
// 00200000          81 80 80 00
// 08000000          C0 80 80 00
// 0FFFFFFF          FF FF FF 7F

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut rb = values[0];
    let mut v = Vec::new();
    let mut idx = 0;
    loop {
        let mut b = rb & 0x7F;
        if idx == 0 {
            b &= 0x7F;
        } else {
            b |= 0x80;
        }
        v.push(b as u8);
        rb >>= 7;
        idx += 1;

        if rb == 0 {
            break;
        }
    }

    v.into_iter().rev().collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, &'static str> {
    unimplemented!()
}
