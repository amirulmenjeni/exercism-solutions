use std::{thread, vec};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    
    let lock = Arc::new(Mutex::new(HashMap::<char, usize>::new()));

    let workers = if input.len() <= worker_count { 1 } else { worker_count };

    let threads: Vec<_> = (0..workers)
        .map(|i| {
            let lock = Arc::clone(&lock);
            let vec_input = input.iter().map(|&s| String::from(s)).collect();

            thread::spawn(move || worker_thread(i, workers, lock, vec_input))

        })
        .collect();

    for t in threads {
        t.join().unwrap();
    }

    let g = lock.lock().unwrap();
    g.clone()
}

fn worker_thread(id: usize, worker_count: usize, lock: Arc<Mutex<HashMap<char, usize>>>, input: Vec<String>) {

    // The start position and the end position (non-inclusive) of the slice this worker thread will work on.
    let range = f64::ceil((input.len() as f64 / worker_count as f64) as f64) as usize;
    let start = id * range;
    let end = std::cmp::min(start + range, input.len());

    input[start..end].iter().for_each(|s| {
        s.chars().for_each(|c| {
            let mut hm = lock.lock().unwrap();
            if c.is_alphabetic() {
                let value = hm.entry(c.to_ascii_lowercase()).or_insert(0);
                *value = *value + 1;
            }
        });
    });
}
