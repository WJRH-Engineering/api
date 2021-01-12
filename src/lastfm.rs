use reqwest as fetch;
use serde::Deserialize;


pub struct Song {
	pub title: String,
	pub artist: String,
	pub details: Option<LastFMTrack>,
}


#[derive(Deserialize, Clone)]
pub struct LastFMArtist {
	pub name: Option<String>,
	pub mbid: Option<String>,
	pub url: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct LastFMWiki {
	pub published: Option<String>,
	pub summary: Option<String>,
	pub content: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct LastFMTrack {
	pub name: Option<String>,
	pub mbid: Option<String>,
	pub url: Option<String>,
	pub duration: Option<String>,
	pub artist: Option<LastFMArtist>,
	pub wiki: Option<LastFMWiki>,
}

#[derive(Deserialize)]
pub struct LastFMResponse {
	pub track: Option<LastFMTrack>,
}


pub async fn lookup_song(title: &str, artist: &str) -> Song {
	let url_base = "ws.audioscrobbler.com/2.0/";	
	let url = format!("http://{base}?{method}&api_key={lastfmkey}&artist={artist}&track={track}&autocorrect&format=json",
		base = url_base,
		lastfmkey = "14cacc2d28210dcd318ffa2085778844",
		method = "method=track.getInfo",
		artist = artist.replace(" ", "+"),
		track = title.replace(" ", "+"),
	);

	let response_string = fetch::get(&url).await.unwrap().text().await.unwrap();
	let response: LastFMResponse = serde_json::from_str(&response_string).unwrap();

	Song {
		title: title.to_string(),
		artist: artist.to_string(),
		details: response.track,
	}
}
