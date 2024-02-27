use std::env;
use serde_json::Value;

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

// Parses the JSON response from the apex legends API.
// Returns a format string containing the relevant data.
pub fn parse_json(json: String) -> Result<String, serde_json::Error> {
    let data: Value = serde_json::from_str(&*json)?;
    // Create references to different parts of the JSON.
    // This allows us to avoid repeatedly copying each part.
    let global: &Value = &data["global"];
    let ranked: &Value = &global["rank"];
    let total: &Value = &data["total"];

    let name: String = global["name"].to_string().replace("\"", "");
    let tag: String = {
        let temp: String = global["tag"].to_string().replace("\"", "");
        if temp == "" {
            temp
        } else {
            format!("[{temp}]")
        }
    };
    let level: String =  global["level"].to_string().replace("\"", "");
    let platform: String = global["platform"].to_string().replace("\"", "");

    let rank: String = format!("{} {}, {} RP",
                               ranked["rankName"].to_string().replace("\"", ""),
                               ranked["rankDiv"], ranked["rankScore"]);

    let ladder_global: String = {
        let top_bottom: String = {
            let global_percentage: f64 = ranked["ALStopPercentGlobal"].as_f64().unwrap();
            if global_percentage < 50.0 {
                format!("top {global_percentage}%")
            } else {
                format!("bottom {global_percentage}%")
            }
        };
        format!("{} ({})", ranked["ALStopIntGlobal"], top_bottom)
    };

    let ladder_platform: String = {
        let top_bottom: String = {
            let platform_percentage: f64 = ranked["ALStopPercent"].as_f64().unwrap();
            if platform_percentage < 50.0 {
                format!("top {platform_percentage}%")
            } else {
                format!("bottom {platform_percentage}%")
            }
        };
        format!("{} ({})", ranked["ALStopInt"], top_bottom)
    };
    let career_kills: i64 = total["career_kills"]["value"].as_i64().unwrap();
    let career_wins : i64 = total["career_wins"]["value"].as_i64().unwrap();
    let career_revives: i64 = total["career_revives"]["value"].as_i64().unwrap();

    let parsed_json : String = format!(
        "Name: {tag}{name}\n\
        Level: {level}\n\
        Rank: {rank}\n\
        Global ladder position: {ladder_global}\n\
        {platform} ladder position : {ladder_platform}\n\
        Career Kills : {career_kills}\n\
        Career Wins: {career_wins}\n\
        Career Revives: {career_revives}
        "
    );

    Ok(parsed_json)
}