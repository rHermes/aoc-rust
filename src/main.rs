use std::collections::HashMap;
use std::convert::TryFrom;
use std::env;

use std::time::Instant;

use reqwest;
use reqwest::header::HeaderValue;

use futures::future::join_all;

use colored::*;

use tokio;

const SESSION_PREFIX: &str = "AOC_SESSION_";

use alloc_counter::AllocCounterSystem;

#[global_allocator]
static A: AllocCounterSystem = AllocCounterSystem;

#[tokio::main]
async fn main() {
    // Get the session keys
    let sess_keys = env::vars_os()
        .filter_map(|(k, v)| {
            k.to_str()
                .filter(|ks| ks.starts_with(SESSION_PREFIX))
                .and_then(move |ks| {
                    v.to_str()
                        .and_then(|vs| {
                            HeaderValue::try_from(format!("session={}", vs))
                                .ok()
                                .and_then(|vv| Some((String::from(vs), vv)))
                        })
                        .and_then(|vs| Some((String::from(&ks[SESSION_PREFIX.len()..]), vs)))
                })
        })
        .collect::<HashMap<_, _>>();

    let thingies = sess_keys.iter().map(|(sess_name, (sess_key, sess_val))| {
        async move {
            let mut headers = reqwest::header::HeaderMap::new();

            headers.insert(reqwest::header::COOKIE, sess_val.clone());

            let client = reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap();

            let tasks = (2020..2021)
                .flat_map(|y| (2..16).map(move |d| (y, d)))
                .filter(|&x| x != (2015, 4)) // Filter out slow task
                .collect::<Vec<_>>();

            let neinei = tasks
                .iter()
                .map(|&(y, d)| aoc::utils::get_input(&client, sess_key, y, d));

            tasks
                .iter()
                .cloned()
                .zip(join_all(neinei).await)
                .map(|((y, d), i)| (sess_name, y, d, i))
                .collect::<Vec<_>>()
        }
    });

    let inputs = join_all(thingies).await;

    let kiwi = inputs.iter().flatten().collect::<Vec<_>>();

    // This is a stupid way to do things, but I don't care
    for k in sess_keys.keys() {
        println!("{}", k);
        for (_, y, d, i) in kiwi.iter().filter(|&(sess_name, _, _, _)| *sess_name == k) {
            if let Err(e) = i {
                println!("  {}-{:02}: {}", y, d, e);
                continue;
            }

            let input = i.as_ref().unwrap();

            let start1 = Instant::now();
            let part1 = match aoc::solutions::run_task(*y, *d, 1, input.as_ref()) {
                Ok(a) => a,
                Err(a) => a,
            };
            let dur1 = start1.elapsed();

            let start2 = Instant::now();
            let part2 = match aoc::solutions::run_task(*y, *d, 2, input.as_ref()) {
                Ok(a) => a,
                Err(a) => a,
            };
            let dur2 = start2.elapsed();

            if part1 != "We don't have that task." || part2 != "We don't have that task." {
                println!("  {}-{:02}:", y, d);
                println!(
                    "    Part 1: {:>10} {:>10} μs",
                    part1.green(),
                    dur1.as_micros().to_string().blue()
                );
                println!(
                    "    Part 2: {:>10} {:>10} μs",
                    part2.green(),
                    dur2.as_micros().to_string().blue()
                );
            }
        }
    }
}
