use again::RetryPolicy;
use clap::{App, Arg};
use reqwest;
use reqwest::StatusCode;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Duration;
use tokio;

// I think the API is broke currently...

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn format_post_request_vec(lat_lons: Vec<(f32, f32)>) -> String {
    let mut formatted_string = String::new();
    for el in lat_lons {
        let each_el = format!("\"{} {}\"", el.0, el.1);
        formatted_string += &each_el;
        formatted_string += ", ";
    }
    let stripped_formatted_string = formatted_string.strip_suffix(", ").unwrap();
    let request = format!(
        "{{\"op\":\"convert\",\"locations\":[{}],\"types\":[\"vice-county\"]}}",
        stripped_formatted_string
    );
    request
}

// get is fallible. Wrap in again::retry

async fn request_url(url: &str, lat_lons: Vec<(f32, f32)>) -> Result<String, Box<dyn Error>> {
    // let post = format_post_request(lat, lon);
    let post = format_post_request_vec(lat_lons);

    let policy: RetryPolicy = RetryPolicy::exponential(Duration::from_millis(5000))
        .with_max_retries(10)
        .with_jitter(false);

    let request = policy
        .retry(|| async {
            let client = reqwest::Client::new();

            let body = client.post(url).body(post.clone()).build().unwrap();
            client.execute(body).await
        })
        .await;
    let text = match request {
        Ok(e) if e.status() == StatusCode::OK => e,
        Ok(e) if e.status() == StatusCode::NOT_FOUND => {
            panic!("{:?}", e);
        }
        Ok(e) => {
            panic!("{:?}", e);
        }
        Err(e) => {
            panic!("{}", e);
        }
    };
    Ok(text.text().await?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("geodojo_county")
        .version(clap::crate_version!())
        .author("Max Brown <mb39@sanger.ac.uk>")
        .about("Get UK counties from lat-long data.")
        .arg(
            Arg::with_name("file")
                .long("file")
                .short("f")
                .takes_value(true)
                .required(true)
                // .allow_hyphen_values(true)
                .help("The input file containing lat long whitespace separated lines."),
        )
        .get_matches();

    // required by clap, so safe to unwrap.
    let lat_lon_file = matches.value_of("file").unwrap();

    // this is really terrible error handling...
    let mut lat_lons = Vec::new();
    if let Ok(lines) = read_lines(lat_lon_file) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let split = ip.split_whitespace().collect::<Vec<&str>>();
                let lat = split[0].parse::<f32>().unwrap();
                let lon = split[1].parse::<f32>().unwrap();
                lat_lons.push((lat, lon));
            }
        }
    }

    let response = request_url("https://geodojo.net/locate/api/", lat_lons).await;
    let v: Value = serde_json::from_str(&response?)?;
    let len = v.as_array().unwrap().len();

    let mut index = 0;

    loop {
        if index == len - 1 {
            break;
        }

        let county_op = v[index]["locations"]["vice-county"].as_str();
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

        let lat_lon_op = v[index]["location"].as_str();
        let lat_lon = match lat_lon_op {
            Some(s) => {
                let split = s.split(' ').collect::<Vec<&str>>();
                (split[0], split[1])
            }
            None => ("", ""),
        };

        println!("{}\t{}\t{}", lat_lon.0, lat_lon.1, county);
        index += 1;
    }

    Ok(())
}
