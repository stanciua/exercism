///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits. However, your function must be able to
///     process input with leading 0 digits.
///
#[allow(unused_variables)]
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, ()> {
    let mut rslt = Vec::new();
    if from_base < 2 || to_base < 2 {
        return Err(());
    }

    if number.is_empty() {
        return Ok(rslt);
    }

    if !number.into_iter().map(|n| *n < from_base).all(|v| v) {
        return Err(());
    }

    let from_number =
        (0..number.len()).into_iter().rev().zip(number.iter()).fold(0, |mut acc, (a, b)| {
            acc += *b as u32 * from_base.pow(a as u32);
            acc
        });

    // it's ok to receive &[0, 0..] slices, but we should not return anything
    // in this case
    if from_number == 0 {
        return Ok(rslt);
    }

    let to_base_high_power = get_to_base_high_power(from_number, to_base);

    let mut rest = from_number;
    for power in (0..to_base_high_power + 1).into_iter().rev() {
        let mult = rest / to_base.pow(power as u32);
        rslt.push(mult);
        rest = from_number % to_base.pow(power as u32);
    }

    Ok(rslt)
}

fn get_to_base_high_power(from_number: u32, to_base: u32) -> u32 {
    (0..64).into_iter().take_while(|p| to_base.pow(*p as u32) <= from_number).last().unwrap_or(0)
}
