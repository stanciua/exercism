pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut idx = 0;
    let mut rb: u32;
    let mut per_stream_vec = Vec::new();
    let mut per_byte_vec: Vec<u8> = Vec::new();
    for value in values {
        rb = *value;

        loop {
            let mut b = rb & 0x7F;
            if idx == 0 {
                b &= 0x7F;
            } else {
                b |= 0x80;
            }
            per_byte_vec.push(b as u8);
            rb >>= 7;
            idx += 1;

            if rb == 0 {
                idx = 0;
                break;
            }
        }
        per_stream_vec.push(per_byte_vec);
        per_byte_vec = Vec::new();
    }

    per_stream_vec.into_iter().flat_map(|val| val.into_iter().rev()).collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, &'static str> {
    let mut v: Vec<u32> = Vec::new();

    let mut no_bytes = 0;
    let mut u32_val = 0u32;
    let mut incomplete_byte_seq = false;
    for byte in bytes {
        if byte & 0x80 != 0 {
            incomplete_byte_seq = true;
            no_bytes += 1;
            u32_val |= (byte & 0x7F) as u32;
            u32_val <<= 7;
        } else {
            incomplete_byte_seq = false;
            no_bytes += 1;
            // check for overflow
            if no_bytes > 4 {
                return Err("value overflowed!");
            }
            u32_val |= (byte & 0x7F) as u32;
            v.push(u32_val);
            u32_val = 0;
            no_bytes = 0;
        }
    }

    if incomplete_byte_seq {
        return Err("incomplete sequence!");
    }

    Ok(v)
}
