const {gql} = require('apollo-server')

module.exports = gql`

type Query {
	programs: [Program]
	program(shortname: String): Program
	schedule: [Timeslot]
}

type Program {
	active: Boolean,
	author: String,
	copyright: String,
	cover_image: String,
	description: String,
	explicit: Boolean,
	image: String,
	itunes_categories: [String],
	language: String,
	name: String,
	organizations: [String],
	owners: [String],
	redirect_url: [String],
	scheduled_time: String,
	shortname: String,
	stream: String,
	subtitle: String,
	tags: String,
	id: String,
	episodes(limit: Int): [Episode]
}

type Episode {
	audio_url: String,
	delay: Int,
	description: String,
	end_time: String,
	explicit: Boolean,
	guid: String,
	hits: Int,
	image: String,
	length: Int,
	name: String,
	pubdate: String,
	start_time: String,
	type: String,
	id: String,
	tracks: [Track]
},

type Track {
	artist: String,
	log_time: String,
	mbid: String,
	title: String,
	id: String
	lastfm_data: LastFM_Data
}

type LastFM_Data {
	name: String
	mbid: String
	duration: Int
	artist: String
	album: String
	artwork: String
	wiki: String
}

type Timeslot {
	program_name: String
	time_range: String
	program: Program
	day_of_week: String
}`
