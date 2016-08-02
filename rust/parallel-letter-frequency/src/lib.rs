extern crate scoped_threadpool;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use scoped_threadpool::Pool;

pub fn frequency(text: &[&str], num_threads: usize) -> HashMap<char, usize> {
    let rslt = Arc::new(Mutex::new(HashMap::<char, usize>::new()));
    let mut pool = Pool::new(num_threads as u32);

    pool.scoped(|scope| {
        for line in text {
            let rslt = rslt.clone();
            scope.execute(move || {
                let rslt_per_line = line.chars()
                    .filter(|ch| ch.is_alphabetic())
                    .fold(HashMap::new(), |mut acc, val| {
                        {
                            let count = acc.entry(val.to_lowercase().next().unwrap()).or_insert(0);
                            *count += 1;
                        }
                        acc
                    });

                {
                    let rslt = rslt.lock().unwrap();
                    let _ = rslt_per_line.iter().fold(rslt, |mut acc, (key, val)| {
                        {
                            let new_val = acc.entry(*key).or_insert(0);
                            *new_val += *val;
                        }
                        acc
                    });
                }

            });
        }


    });

    let data = rslt.lock().unwrap();
    data.clone().into_iter().collect()

}
