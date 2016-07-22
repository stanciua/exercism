pub fn square_of_sum(nums: i32) -> i32 {
    (1..nums+1).into_iter().fold(0, |mut acc, val| {acc += val; acc}).pow(2)
}

pub fn sum_of_squares(nums: i32) -> i32 {
    (1..nums+1).into_iter().fold(0, |mut acc, val| {acc += val.pow(2); acc})
}

pub fn difference(nums: i32) -> i32 {
    square_of_sum(nums) - sum_of_squares(nums)
}