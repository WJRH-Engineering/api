use serde::Deserialize;
use surf;

pub async fn lookup_song_name(title: &str, artist: &str) -> Response {
    let url = format!("http://{base}?{method}&api_key={lastfmkey}&artist={artist}&track={track}&autocorrect&format=json",
        base = "ws.audioscrobbler.com/2.0/", 
        lastfmkey = "14cacc2d28210dcd318ffa2085778844",
        method = "method=track.getInfo",
        artist = artist.replace(" ", "+"),
        track = title.replace(" ", "+"),
    );
    
    surf::get(&url).recv_json().await.unwrap()
}

#[derive(Deserialize, Debug)]
pub struct Response {
	pub track: Option<Track>,
}

#[derive(Deserialize, Default, Debug)]
pub struct Track {
	pub name: String,
	pub mbid: Option<String>,
	pub url: Option<String>,
	pub duration: Option<String>,
	pub artist: Option<Artist>,
	pub wiki: Option<Wiki>,
    pub album: Option<Album>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Album {
    pub artist: Option<String>,
    pub title: Option<String>,
    pub mbid: Option<String>,
    pub lastfm_url: Option<String>,
    pub image: Vec<Image>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Image {
    #[serde(rename = "#text")]
    pub url: String,
    pub size: String,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Artist {
	pub name: String,
	pub mbid: Option<String>,
	pub url: Option<String>,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Wiki {
	pub published: Option<String>,
	pub summary: Option<String>,
	pub content: Option<String>,
}

 #[cfg(test)]
 mod tests {
	use super::*;

	#[async_std::test]	
	async fn deserialize_lastfm_response() {
	    let response = lookup_song_name("the distance", "cake").await;	
        println!("{}", response.track.unwrap().album.unwrap().image[2].url);
	}
 }
