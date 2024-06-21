use reqwest::blocking::Client;
use reqwest::Error;

fn main() -> Result<(), Error> {
    let client = Client::new();
    let user = "testuser".to_string();
    let password: Option<String>= None;

    let response= client
    .get("https://www.mediawiki.org/wiki/API:Main_page?ref=public_apis")
    .basic_auth(user, password)
    .send();

    println!("{:?}", response);
    Ok(())
}
    