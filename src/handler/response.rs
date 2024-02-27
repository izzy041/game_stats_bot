use std::env;
use std::str;

// Makes a GET request to the games API.
// Returns a Result containing the response body as the Ok value
// and a reqwest::Error as the Err value.
pub async fn get_response(client: &reqwest::Client, player: &String) -> Result<String, reqwest::Error> {
    let api_key: String = env::var("APEX_API_KEY").expect("no api key provided");
    let url: String = format!("https://api.mozambiquehe.re/bridge?auth={api_key}&player={player}&platform=PC");

    let request: reqwest::Request = client.request(reqwest::Method::GET, url)
        .build()?;
    let response: String = client.execute(request)
        .await?
        .text()
        .await?;

    Ok(response)
}

// Splits the response into a vector of string slices
pub fn split_response (response: &String) -> Vec<&str> {
    let slice: usize = response.len() / 5;
    let substr = response.as_bytes()
        .chunks(slice)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();
    return substr
}