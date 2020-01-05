// 2015 Day 4
//
// This solution is inspired by
// https://gist.github.com/SimonWoodburyForget/075db6ceb2d283b0ad5f
//
// First time using multithreading and rust together. Let's see how
// it goes

use std::io::Write;
use std::str;

use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;

use md5::{Digest, Md5};

use num_cpus;

pub fn part1(input: &[u8]) -> Result<String, String> {
    // Remove newline
    let rinput = input
        .iter()
        .take_while(|&&x| x != b'\n')
        .copied()
        .collect::<Vec<u8>>();

    let sinput = str::from_utf8(&rinput).map_err(|e| e.to_string())?;
    let ans = find_hashes(sinput, false).map(|x| x.to_string());
    ans.ok_or("No such hash found!".to_string())
}

pub fn part2(input: &[u8]) -> Result<String, String> {
    // Remove newline
    let rinput = input
        .iter()
        .take_while(|&&x| x != b'\n')
        .copied()
        .collect::<Vec<u8>>();

    let sinput = str::from_utf8(&rinput).map_err(|e| e.to_string())?;
    let ans = find_hashes(sinput, true).map(|x| x.to_string());
    ans.ok_or("No such hash found!".to_string())
}

struct Data {
    // Solutions we want
    n: u8,
    // keeps track of the solutions tried
    i: u64,
}

fn find_hashes(input_begin: &str, part2: bool) -> Option<u64> {
    // Arc is referenced counted atomic.
    let input_begin = Arc::new(input_begin.to_string());
    let data = Arc::new(Mutex::new(Data { n: 1, i: 1 }));

    let threads = num_cpus::get();
    let (tx, rx) = channel();

    for _ in 0..threads {
        // This is the amount of work we will do per iteration.
        let buffer_size = (threads as u64) * 30;

        let (data, tx) = (data.clone(), tx.clone());
        let input_begin = input_begin.clone();

        let mut buf = Vec::with_capacity(8);
        let mut hasher = Md5::new();
        thread::spawn(move || {
            let key = input_begin.as_bytes();

            // The state of count when the thread last saw it
            let mut count;
            let mut passed_test = false;

            // this is the main loop
            loop {
                // we are only going to need the lock for a short amount
                // of time so we scope it
                {
                    let mut data = data.lock().unwrap();

                    if data.n == 0u8 {
                        break;
                    }

                    if passed_test {
                        passed_test = false;
                        // TODO(rHermes): Couldn't this wrap around potentially?
                        data.n -= 1;
                    }

                    // concurrent amount done and planned
                    count = data.i;

                    data.i += buffer_size;
                }

                for _ in 0..buffer_size {
                    // we add by one before starting, because the task asks for
                    // smallest postivie number
                    buf.clear();
                    write!(buf, "{}", count).unwrap();

                    hasher.input(&key);
                    hasher.input(&buf);

                    let dgst = hasher.result_reset();

                    let kv = if !part2 && dgst[..2] == [0, 0] && dgst[2] < 0x10 {
                        Some(count)
                    } else if part2 && dgst[..3] == [0, 0, 0] {
                        Some(count)
                    } else {
                        None
                    };

                    if let Some(x) = kv {
                        passed_test = true;
                        tx.send(x).unwrap();
                        break;
                    }
                    count += 1;
                }
            }
        });
    }

    // We need to drop the tx here to close the channel completly
    drop(tx);

    // we might get more answers from the threads, so we
    // listen for more
    let mut ans: Option<u64> = None;
    loop {
        match rx.recv() {
            Ok(e) => {
                ans = ans.map_or(Some(e), |v| Some(v.min(e)));
            }
            _ => break,
        }
    }

    ans
}
