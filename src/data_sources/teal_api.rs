/// --------
/// TEAL API
/// --------
/// This module deals with fetching data from teal's http api. It is primarily
/// used for fetching track data for specific episodes, because it would be 
/// too tedious and wasteful to store these in the redis cache

use surf;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct TealEpisode {
    id: String,
    name: String,
    description: String,
    audio_url: String,
    
    pubdate: String,
    start_time: String,
    end_time: String,

    delay: i32,
    guid: String,
    hits: i32,
    length: String,

    #[serde(rename = "type")]
    file_type: String,

    tracks: Vec<TealTrack>, 
}

#[derive(Deserialize, Clone, Debug)]
pub struct TealTrack {
	pub title: Option<String>,
	pub artist: Option<String>,
	pub log_time: Option<String>,
	pub mbid: Option<String>,
	pub id: Option<String>,
}

pub async fn get_tracks(episode_id: &str) -> Vec<TealTrack> {
    let url = format!("https://api.teal.cool/episodes/{}", episode_id.to_string());
    let res: String = surf::get(&url).recv_string().await.unwrap();

    // println!("{}", res);
    let response: TealEpisode = surf::get(&url).recv_json().await.unwrap();

    return response.tracks.clone();
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[async_std::test]
    async fn query(){
        let tracks = get_tracks("5ab453c48db8510012b15ddb").await;
        // panic!("{:#?}", tracks);
   } 
}
