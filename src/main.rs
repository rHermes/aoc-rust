use std::collections::HashMap;
use std::convert::TryFrom;
use std::env;

use reqwest;
use reqwest::header::HeaderValue;

use futures::future::join_all;

use tokio;

const SESSION_PREFIX: &str = "AOC_SESSION_";

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

            // let tasks = (2015..2020).flat_map(|y| (1..26).map(move |d| (y, d))).collect::<Vec<_>>();
            let tasks = (2015..2016)
                .flat_map(|y| (1..26).map(move |d| (y, d)))
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

            let part1 = match aoc::solutions::run_task(*y, *d, 1, input.as_ref()) {
                Ok(a) => a,
                Err(a) => a,
            };
            let part2 = match aoc::solutions::run_task(*y, *d, 2, input.as_ref()) {
                Ok(a) => a,
                Err(a) => a,
            };

            if part1 != "We don't have that task." || part2 != "We don't have that task." {
                println!("  {}-{:02}:", y, d);
                println!("    Part 1: {}", part1);
                println!("    Part 2: {}", part2);
            }
        }
    }
}
