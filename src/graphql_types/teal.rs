use async_graphql::*;
use crate::data_sources::redis;
use crate::data_sources::teal_api;

/// Metadata about a program on teal
#[derive(SimpleObject, Default)]
#[graphql(complex)]
pub struct Program {
    pub name: String,
    pub subtitle: Option<String>,
    pub author: String,
    pub id: String,

    pub image: String,
    pub cover_image: String,

    pub active: bool,
    pub copyright: String,
    pub description: String,
    pub explicit: bool,
    pub language: String,

    pub itunes_categories: Vec<String>,
    pub organizations: Vec<String>,
    pub owners: Vec<String>,
    pub tags: Vec<String>,

    pub redirect_url: Option<String>,

    #[graphql(skip)]
    pub episode_ids: Vec<String>,
}

#[ComplexObject]
impl Program {
    pub async fn episodes (&self) -> Vec<Episode> {
        let mut output: Vec<Episode> = vec![];
        for id in &self.episode_ids {
            output.push(redis::Episode::get(&id).into())
        }

        output
    }  
}

#[derive(SimpleObject, Default)]
#[graphql(complex)]
pub struct Episode {
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
    length: i32,
    file_type: String,
}

#[ComplexObject]
impl Episode {
    pub async fn tracks(&self) -> Vec<TrackLog> {
        todo!()
    }  
}

/// A TrackLog is a wrapper around a Song with extra metadata about when it was
/// played
#[derive(SimpleObject, Default)]
#[graphql(complex)]
pub struct TrackLog {

    #[graphql(skip)]
	pub title: Option<String>,
    #[graphql(skip)]
	pub artist: Option<String>,
    #[graphql(skip)]
	pub mbid: Option<String>,

	pub log_time: Option<String>,
	pub id: Option<String>,
}

#[ComplexObject]
impl TrackLog {
    // pub async fn song()
}


#[derive(Default)]
pub struct TealQuery;

#[Object]
impl TealQuery {

	/// Get all of the programs in teal
	pub async fn programs(&self, limit: Option<usize>) -> Vec<Program> {
        let programs = redis::Program::get_all();

        // Convert the list of programs from redis::Program to teal::Program
        let mut converted_programs: Vec<Program> = vec![];
        for program in programs {
            converted_programs.push(program.into());
        }

        if let Some(limit) = limit {
            converted_programs.truncate(limit);
        }

        converted_programs
	}

    pub async fn program(&self, shortname: String) -> Program {
        redis::Program::get(&shortname).into()
    }
}


// ----------------
// TYPE CONVERSIONS
// ----------------

impl From<redis::Program> for Program {
    fn from(program: redis::Program) -> Self {
        Program {
            name: program.scalar("name"),
            subtitle: program.optional_scalar("subtitle"),
            author: program.scalar("author"),
            id: program.scalar("id"),
            image: program.scalar("image"),
            cover_image: program.scalar("cover_image"),

            // active: program.scalar("name"),
            copyright: program.scalar("copyright"),
            description: program.scalar("description"),
            // explicit: program.scalar("name"),
            language: program.scalar("language"),
            
            redirect_url: program.optional_scalar("redirect_url"),


            // Episode Ids
            episode_ids: program.episode_ids,

            ..Self::default()
        } 
    }
}

impl From<redis::Episode> for Episode {
    fn from(episode: redis::Episode) -> Self {
        Episode {
            id: episode.id.clone(),
            name: episode.scalar("name"),
            description: episode.scalar("description"),
            audio_url: episode.scalar("audio_url"),

            pubdate: episode.scalar("pubdate"),
            start_time: episode.scalar("start_time"),
            end_time: episode.scalar("end_time"),

            // delay: episode.scalar("delay").parse::<i32>().unwrap(),
            guid: episode.scalar("guid"),
            // hits: episode.scalar("hits").parse::<i32>().unwrap(),
            // length: episode.scalar("length").parse::<i32>().unwrap(),
            file_type: episode.scalar("file_type"),

            ..Self::default()
        }
    }
}

impl From<teal_api::TealTrack> for TrackLog {
    fn from(track: teal_api::TealTrack) -> Self {
        todo!();
    }
}
