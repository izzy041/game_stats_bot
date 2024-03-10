use serde_json::Value;
use std::env;
use serenity::all::{Colour, CreateEmbed, CreateEmbedAuthor, CreateMessage, User};
use serenity::builder::CreateEmbedFooter;
use serenity::model::Timestamp;

// Makes a GET request to the games API.
// Returns a Result containing the response body as the Ok value
// and a reqwest::Error as the Err value.
pub async fn get_response(client: &reqwest::Client, player: &String, platform: &String) -> Result<String, reqwest::Error> {
    let api_key: String = env::var("APEX_API_KEY").expect("no api key provided");
    let url: String =
        format!("https://api.mozambiquehe.re/bridge?auth={api_key}&player={player}&platform={platform}");

    let request: reqwest::Request = client.request(reqwest::Method::GET, url).build()?;
    let response: String = client.execute(request).await?.text().await?;

    Ok(response)
}

// Struct containing player stats
pub struct PlayerStats {
    pub name: String,
    pub level: i64,
    pub rank: String,
    pub rank_image: String,
    pub global_ladder_position: String,
    pub platform: String,
    pub platform_ladder_position: String,
    pub career_kills: i64,
    pub career_wins: i64,
    pub career_revives: i64,
}

impl PlayerStats {
    // Function to get the stats as a CreateMessage type.
    pub fn as_message(&self, requester: &User) -> CreateMessage {
        // The embed used if the player was found.
        let success_embed = CreateEmbed::new()
            .title(&self.name)
            .fields(vec![
                ("Level", &self.level.to_string(), false),
                ("Rank", &self.rank, false),
                ("Global Ladder Position", &self.global_ladder_position, false),
                (format!("{} ladder position", &self.platform).as_str(), &self.platform_ladder_position, false),
                ("Career Kills", &self.career_kills.to_string(), false),
                ("Career Wins", &self.career_wins.to_string(), false),
                ("Career Revives", &self.career_revives.to_string(), false),
            ])
            .author(CreateEmbedAuthor::new(format!("@{}", &requester.name)))
            .color(Colour::TEAL)
            .footer(CreateEmbedFooter::new(format!("Apex legends statistics for player \"{}\", requested by {}.", self.name, requester.name)))
            .timestamp(Timestamp::now())
            .thumbnail(&self.rank_image);

        // The embed used if the player was not found.
        // Contains an error message.
        let fail_embed = CreateEmbed::new()
            .title("Error getting Apex Legends Statistics")
            .description("Player was not found")
            .footer(CreateEmbedFooter::new(format!("Apex legends statistics requested by {}.", &requester.name)))
            .timestamp(Timestamp::now())
            .color(Colour::RED);

        // If the level is 0 then the player was not found,
        // otherwise, the message is returned.
        match self.level {
            0 => CreateMessage::new().embed(fail_embed),
            _ => CreateMessage::new().embed(success_embed),
        }
    }
}


// Parses the JSON response from the apex legends API.
// Returns a format string containing the relevant data.
pub fn parse_json(json: &str) -> Result<PlayerStats, serde_json::Error> {
    let data: Value = serde_json::from_str(json)?;
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
    let level: i64 = global["level"]
        .to_string()
        .replace("\"", "")
        .parse()
        .unwrap_or(0);
    let platform: String = global["platform"].to_string().replace("\"", "");

    let rank: String = format!(
        "{} {}, {} RP",
        ranked["rankName"].to_string().replace("\"", ""),
        ranked["rankDiv"],
        ranked["rankScore"]
    );
    let rank_image: String = ranked["rankImg"].to_string().replace("\"", "");

    let ladder_global: String = {
        let top_bottom: String = {
            let global_percentage: f64 = ranked["ALStopPercentGlobal"].as_f64().unwrap_or(0.0);
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
            let platform_percentage: f64 = ranked["ALStopPercent"].as_f64().unwrap_or(0.0);
            if platform_percentage < 50.0 {
                format!("top {platform_percentage}%")
            } else {
                format!("bottom {platform_percentage}%")
            }
        };
        format!("{} ({})", ranked["ALStopInt"], top_bottom)
    };
    let career_kills: i64 = total["career_kills"]["value"].as_i64().unwrap_or(0);
    let career_wins: i64 = total["career_wins"]["value"].as_i64().unwrap_or(0);
    let career_revives: i64 = total["career_revives"]["value"].as_i64().unwrap_or(0);

    let stats: PlayerStats = PlayerStats {
        name: format!("{tag}{name}"),
        level,
        rank,
        global_ladder_position: ladder_global,
        platform,
        platform_ladder_position: ladder_platform,
        career_kills,
        career_wins,
        career_revives,
        rank_image
    };

    Ok(stats)
}