extern crate itertools;

use itertools::Itertools;

pub fn verse(num: i32) -> String {
    match num {
        0 => "No more bottles kl no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_owned(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_owned(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_owned(),
        3...99 => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", num, num, num - 1),
        _ => panic!("Unsupported number of beers!!!") 
    }
}

pub fn sing(num1: i32, num2: i32) -> String {  
    (num2 .. num1 + 1).into_iter().rev()
                                  .fold(String::new(), |mut acc, x| {acc.push_str(&verse(x)); acc.push_str("\n"); acc})
                                  .chars()
                                  .dropping_back(1)
                                  .collect()

}
