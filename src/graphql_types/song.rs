use crate::data_sources::lastfm;
use async_graphql::*;

#[derive(Default, SimpleObject)]
#[graphql(complex)]
/// Metadata about a song pulled from the LastFM api
pub struct SongDetails {

	pub title: String,
    pub artist: String,
    pub album: Option<String>,

    /// The Song's unique Music Brainz ID
    /// https://musicbrainz.org/doc/MusicBrainz_Identifier
	pub mbid: String,
    
    /// A link to the Song's LastFM page
	pub url: Option<String>,

    /// The Artist's unique Music Brainz ID
    /// https://musicbrainz.org/doc/MusicBrainz_Identifier
    pub artist_mbid: String,

    /// A link to the Artist's LastFM page
    pub artist_url: Option<String>,


    /// More information about the song
    pub wiki: Option<String>,

    /// A briefer summary of the wiki
    pub wiki_summary: Option<String>,

    /// The date the wiki was published
    pub wiki_date: Option<String>,

	pub duration: Option<String>,

    #[graphql(skip)]
    image: Image,
}

#[ComplexObject]
impl SongDetails {
    pub async fn image(&self) -> Option<String> {
        self.image.large.clone()
    }
}

/// A reference to a song, which can be used to search for more information
/// from the LastFM database. Songs can be referenced with either an artist
/// and title pair, or with their Music Brainz ID.
enum Song {
    Name { artist: String, title: String },
    MBID(String), // a music brainz id
}

#[Object]
impl Song {
    pub async fn details(&self) -> SongDetails {
        match self {
            Song::Name { title, artist } => {
                lastfm::lookup_song_name(title, artist)
                    .await.track.unwrap().into()
            },
            Song::MBID(id) => panic!("music brainz ids aren't supported yet"),
        }
    }
}

#[derive(Default)]
struct Image { 
    small: Option<String>,
    medium: Option<String>,
    large: Option<String>,
    extra_large: Option<String>,
}


#[derive(Default)]
pub struct SongQuery;

#[Object]
impl SongQuery {
    pub async fn lookup_song(&self, title: String, artist: String) -> SongDetails {
        lastfm::lookup_song_name(&title, &artist).await.track.unwrap().into()
    }
}


// ----------------
// TYPE CONVERSIONS
// ----------------
impl From<Vec<lastfm::Image>> for Image {
    fn from(image: Vec<lastfm::Image>) -> Self {
        Image {
            small: Some(image[0].url.clone()),
            medium: Some(image[1].url.clone()),
            large: Some(image[2].url.clone()),
            extra_large: Some(image[3].url.clone()),
        }
    }
}

impl From<lastfm::Track> for SongDetails {
    fn from(track: lastfm::Track) -> Self {

        let artist = track.artist.unwrap_or_default();
        let wiki = track.wiki.unwrap_or_default();
        let album = track.album.unwrap_or_default();

        SongDetails {
            title: track.name,
            artist: artist.name,
            album: album.title,

            mbid: track.mbid.unwrap_or_default(),
            url: track.url,

            artist_mbid: artist.mbid.unwrap_or_default(),
            artist_url: artist.url,

            wiki: wiki.content,
            wiki_summary: wiki.summary,
            wiki_date: wiki.published,

            duration: track.duration,

            image: album.image.into(),
            
            ..SongDetails::default()
        }
    }
}

