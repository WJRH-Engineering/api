const fetch = require('node-fetch')
const redis = require('ioredis')
const data = new redis(6379, 'api.wjrh.org')

const schema = `
extend type Query {
	programs: [Program]
	program(shortname: String): Program
#	schedule: [Timeslot]
}

extend type Timeslot{
	program: Program
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
#	lastfm_data: LastFM_Data
}
`

const Query = {
	//TODO: shorten when logging is no longer needed
	programs: async function(){
		const shortnames = await data.smembers('programs')
		const output = shortnames.map(async function(shortname){
			const scalars = await data.hgetall(`program:${shortname}:scalars`)
			return scalars
		})
		return output
	},
	program: async function(parent, {shortname}){
		return await data.hgetall(`program:${shortname}:scalars`)
	}
}

const Timeslot = {
	program: async function({shortname}){
		return data.hgetall(`program:${shortname}:scalars`)		
	}
}

const Program = {
	episodes: async function(parent, args){
		const episode_ids = await data.smembers(`program:${parent.shortname}:episodes`)	
		let output = episode_ids.map(id => data.hgetall(`episode:${id}:scalars`))
		
		if(args.limit){
			output = output.slice(0, args.limit)
		}

		return await Promise.all(output)
	}
}

const Episode = {
	tracks: async function(parent){
		const request = fetch(`https://api.teal.cool/episodes/${parent.id}`)		
			.then(res => res.json())
			.catch(err => console.log(err))
		const result = await request
		return result ? result.tracks : []
	}
}

module.exports = {schema, resolvers:{Query, Program, Episode, Timeslot}}
