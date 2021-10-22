// Query the https://www.geodojo.net/uk/grid/info/ API
// return

use std::ops::Deref;

use clap::{value_t, App, Arg};
use quick_xml::events::{BytesText, Event};
use quick_xml::Reader;
use reqwest;
use tokio;

fn lat_long_to_url(lat: f32, lon: f32) -> String {
    let formatted = format!(
        "https://www.geodojo.net/uk/grid/?location={}+{}&dataset=vice_county&encoding=xml",
        lat, lon
    );
    formatted
}

// get is fallible. Wrap in again::retry

async fn request_url(url: &str) -> String {
    let body = again::retry(|| reqwest::get(url))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    body
}

#[tokio::main]
async fn main() {
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

    let url = lat_long_to_url(lat, lon);

    // this is an xml
    let body = request_url(&url).await;

    // parse the xml
    let mut reader = Reader::from_str(&body);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut vcname: Option<String> = None;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name() {
                _name => vcname = Some(String::from_utf8(e.name().to_vec()).unwrap()),
            },
            Ok(Event::Text(e)) => match vcname {
                Some(ref s) => {
                    if s == "vcname" {
                        let escaped_string = BytesText::unescaped(&e).unwrap();
                        let string = std::str::from_utf8(escaped_string.deref()).unwrap();
                        println!("{}", string)
                    }
                }
                None => (),
            },
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            _ => (),
        }
        buf.clear();
    }
}
