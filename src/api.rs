extern crate reqwest;
use reqwest::header;

use crate::model::Root;

pub fn fetch_bikes(
    top_right: (f64, f64),
    bottom_left: (f64, f64),
) -> Result<Root, Box<dyn std::error::Error>> {
    let params = [
        ("top_right", format!("{},{}", top_right.0, top_right.1)),
        (
            "bottom_left",
            format!("{},{}", bottom_left.0, bottom_left.1),
        ),
        ("filter_type", "box".to_string()),
    ];

    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Accept",
        "application/com.donkeyrepublic.v7".parse().unwrap(),
    );

    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let res = client
        .get("https://stables.donkey.bike/api/public/nearby")
        .headers(headers)
        .query(&params)
        .send()?
        .text()?;

    let root = serde_json::from_str(&res)?;
    Ok(root)
}
