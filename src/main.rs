use clap::{value_t, App, Arg};
use reqwest;
use serde_json::Value;
use std::error::Error;
use tokio;

// make a small request JSON
fn format_post_request(lat: f32, lon: f32) -> String {
    let request = format!(
        "{{\"op\":\"convert\",\"locations\":[\"{} {}\"],\"types\":[\"vice-county\"]}}",
        lat, lon
    );
    request
}

// get is fallible. Wrap in again::retry

async fn request_url(url: &str, lat: f32, lon: f32) -> String {
    let post = format_post_request(lat, lon);
    let client = reqwest::Client::new();

    let body = client
        .post(url)
        .body(post)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    body
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("geodojo_county")
        .version(clap::crate_version!())
        .author("Max Brown <mb39@sanger.ac.uk>")
        .about("Get UK counties from lat-long data.")
        .arg(
            Arg::with_name("lat")
                .long("lat")
                .takes_value(true)
                .required(true)
                .help("The input latitude."),
        )
        .arg(
            Arg::with_name("lon")
                .long("lon")
                .takes_value(true)
                .required(true)
                .allow_hyphen_values(true)
                .help("The input longitude."),
        )
        .get_matches();

    let lat = value_t!(matches.value_of("lat"), f32).unwrap_or_else(|e| e.exit());
    let lon = value_t!(matches.value_of("lon"), f32).unwrap_or_else(|e| e.exit());

    let response = request_url("https://geodojo.net/convert/api/", lat, lon).await;
    let v: Value = serde_json::from_str(&response)?;

    let county_op = v[0]["locations"][0]["location"].as_str();
    let county = match county_op {
        Some(s) => {
            let pos = s.chars().position(|x| x == ' ');
            match pos {
                Some(p) => &s[p + 1..],
                None => "",
            }
        }
        None => "",
    };

    println!("{}", county);

    Ok(())
}
