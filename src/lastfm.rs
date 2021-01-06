use reqwest as fetch;
use serde::Deserialize;


pub struct Song {
	pub title: String,
	pub artist: String,
	pub details: Option<LastFMTrack>,
}


#[derive(Deserialize)]
pub struct LastFMArtist {
	pub name: String,
	pub mbid: String,
	pub url: String,
}

#[derive(Deserialize)]
pub struct LastFMTrack {
	pub name: String,
	pub mbid: String,
	pub url: String,
	pub duration: String,
	pub artist: LastFMArtist,
}

#[derive(Deserialize)]
pub struct LastFMResponse {
	pub track: Option<LastFMTrack>,
}


pub async fn lookup_song(title: &str, artist: &str) {
	let url_base = "ws.audioscrobbler.com/2.0/";	
	let url = format!("http://{base}?{method}&api_key={lastfmkey}&artist={artist}&track={track}&autocorrect&format=json",
		base = url_base,
		lastfmkey = "14cacc2d28210dcd318ffa2085778844",
		method = "method=track.getInfo",
		artist = artist,
		track = title,
	);

	let response_string = fetch::get(&url).await.unwrap().text().await.unwrap();
	let response: LastFMResponse = serde_json::from_str(&response_string).unwrap();
	let track: LastFMTrack = match response.track {
		Some(track) => track,
		None => panic!(),
	};

	println!("{}", track.artist.name);
	
}
