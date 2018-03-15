extern crate rustc_serialize;

use rustc_serialize::Encodable;
use rustc_serialize::json::{self, Encoder};

#[derive(RustcEncodable, RustcDecodable)]
struct User {
    name: String,
    post_count: u32,
    likes_burgers: bool,
    avatar: Option<Photo>,
}

#[derive(RustcEncodable, RustcDecodable)]
struct Photo {
    url: String,
    dimensions: (u32, u32),
}

fn main() {
    // Primitives
    println!("{:?}", json::encode(&42));
    println!("{:?}",
        json::encode(&vec!["to", "be", "or", "not", "to", "be"]));
    println!("{:?}", json::encode(&Some(true)));

    // Automatic deserialization
    let user = User {
        name: "Zbyszek".to_string(),
        post_count: 100u32,
        likes_burgers: true,
        avatar: Some(Photo {
            url: "http://lorempixel.com/160/160/".to_string(),
            dimensions: (160u32, 160u32),
        }),
    };
    println!("{:?}", json::encode(&user));

    // Pretty printing
    let mut encoded = String::new();
    {
        let mut encoder = Encoder::new_pretty(&mut encoded);
        user.encode(&mut encoder).expect("JSON encode error");
    }
    println!("{}", encoded);

    // Decoding
    let incoming_request = "{\"name\":\"John\",\"post_count\":2,\"likes_burgers\":false,\"avatar\":null}";
    let decoded: User = json::decode(incoming_request).unwrap();
    println!("My name is {} and I {} burgers",
        decoded.name,
        if decoded.likes_burgers {
            "love"
        } else {
            "don't like"
        });
    assert!(decoded.avatar.is_none());
    let new_request = "{\"id\":64,\"title\":\"24days\",\"stats\":{\"pageviews\":1500}}";
    if let Ok(request_json) = json::Json::from_str(new_request) {
        if let Some(stats) = request_json.find("stats") {
            if let Some(pageviews) = stats.find("pageviews") {
                println!("Pageviews: {}", pageviews);
            }
        }
    }
}
