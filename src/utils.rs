use reqwest;
use bytes::Bytes;

pub async fn run_task(c: reqwest::Client, year: i32, day: i32) -> Result<bool, reqwest::Error> {
    Result::Ok(false)
}

pub async fn get_input(c: reqwest::Client, year: i32, day: i32) -> Result<Bytes,reqwest::Error> {
    reqwest::get("http://httpbin.org/ip")
        .await?
        .bytes()
        .await
}
