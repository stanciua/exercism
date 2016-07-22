pub fn raindrops(num: i32) -> String {
  let mut out = String::new();
  match num {
    _ if num % (3 * 5 * 7) == 0 => out.push_str("PlingPlangPlong"),
    _ if num % (3 * 5)     == 0 => out.push_str("PlingPlang"),
    _ if num % (3 * 7)     == 0 => out.push_str("PlingPlong"),
    _ if num % (5 * 7)     == 0 => out.push_str("PlangPlong"),
    _ if num % 3           == 0 => out.push_str("Pling"),
    _ if num % 5           == 0 => out.push_str("Plang"),
    _ if num % 7           == 0 => out.push_str("Plong"),
    _                           => out.push_str(&format!("{}", num))
  };
  out
}
