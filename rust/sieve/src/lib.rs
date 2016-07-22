#![feature(step_by)]

pub fn primes_up_to(limit: i32) -> Vec<i32> {
    let mut v = vec![true; (limit + 1) as usize];

    for p in 2..limit + 1 {
        if !v[p as usize] {
            continue;
        }
        
        (p..limit + 1).step_by(p).skip_while(|mul| *mul == p).map(|mul| {v[mul as usize] = false; mul}).count();
    };

    v.into_iter().enumerate().skip(2).filter(|&(_, val)| val).map(|(idx, _)| idx as i32).collect()
}