// Roman Numerals
// Write a function to convert from normal numbers to Roman Numerals: e.g.

// The Romans were a clever bunch. They conquered most of Europe and ruled it for hundreds of years. They invented concrete and straight roads and even bikinis. One thing they never discovered though was the number zero. This made writing and dating extensive histories of their exploits slightly more challenging, but the system of numbers they came up with is still in use today. For example the BBC uses Roman numerals to date their programmes.

// The Romans wrote numbers using letters - I, V, X, L, C, D, M. (notice these letters have lots of straight lines and are hence easy to hack into stone tablets).

// 1
// 2
// 3
//  1  => I
// 10  => X
//  7  => VII
// There is no need to be able to convert numbers larger than about 3000. (The Romans themselves didn't tend to go any higher)

// Wikipedia says: Modern Roman numerals ... are written by expressing each digit separately starting with the left most digit and skipping any digit with a value of zero.

// To see this in practice, consider the example of 1990.

// In Roman numerals 1990 is MCMXC:

// 1000=M 900=CM 90=XC

// 2008 is written as MMVIII:

// 2000=MM 8=VIII
use std::collections::HashMap;

fn get_arabic_to_roman_conv_table() -> HashMap<i32, &'static str> {
    vec![// 1..9
         (1, "I"),
         (2, "II"),
         (3, "III"),
         (4, "IV"),
         (5, "V"),
         (6, "VI"),
         (7, "VII"),
         (8, "VIII"),
         (9, "IX"),
         (10, "X"),
         (20, "XX"),
         (30, "XXX"),
         (40, "XL"),
         (50, "L"),
         (60, "LX"),
         (70, "LXX"),
         (80, "LXXX"),
         (90, "XC"),
         (100, "C"),
         (200, "CC"),
         (300, "CCC"),
         (400, "CD"),
         (500, "D"),
         (600, "DC"),
         (700, "DCC"),
         (800, "DCCC"),
         (900, "CM"),
         (1000, "M"),
         (2000, "MM"),
         (3000, "MMM")]
        .into_iter()
        .collect()
}

pub struct Roman;

impl<'a> Roman {
    pub fn from(arabic: i32) -> String {
        assert!(arabic <= 3000);

        // format tousands
        let thousands = arabic / 1000;
        let mut rest = arabic;
        let mut roman_string = String::new();

        let mapping_table = get_arabic_to_roman_conv_table();
        if thousands > 0 {
            rest %= 1000;
            roman_string.push_str(mapping_table[&(thousands * 1000)]);
        }
        let hundreds = rest / 100;
        if hundreds > 0 {
            rest %= 100;

            roman_string.push_str(mapping_table[&(hundreds * 100)]);

        }

        let tenths = rest / 10;
        if tenths > 0 {
            rest %= 10;
            roman_string.push_str(mapping_table[&(tenths * 10)]);

        }
        let single_digits = rest;

        if single_digits > 0 {
            roman_string.push_str(mapping_table[&single_digits]);
        }

        roman_string
    }
}
