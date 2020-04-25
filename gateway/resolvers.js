const fetch = require('node-fetch')
const redis = require('ioredis')
const data = new redis()

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

const {oneLineTrim} = require('common-tags')
const LASTFM_API_KEY = "14cacc2d28210dcd318ffa2085778844"
const resolve_song = async function(artist, title){
	artist = artist || ''
	title = title || ''

	URL = oneLineTrim`
		http://ws.audioscrobbler.com/2.0/?method=track.getInfo
		&api_key=${LASTFM_API_KEY}
		&artist=${encodeURI(artist.replace(" ", "+"))}
		&track=${encodeURI(title.replace(" ", "+"))}
		&autocorrect
		&format=json`

	request = fetch(URL)
		.then(res => res.json())
		.catch(err => null)
	
	const result = (await request).track

	if(result == null) return {}

	let output = {}
	
	if(result.artist){
		output.artist = result.artist.name	
	}
	
	if(result.album){
		output.album = result.album.name
		output.artwork = result.album.image[2]["#text"]
	}
	
	if(result.wiki){
		output.wiki = result.wiki.summary
	}
	
	output = {...result, ...output}
	return output
}


const resolvers = {
	Query, Program, Episode,
	Track: {
		song: parent => resolve_song(parent.artist, parent.title)
	}
}

module.exports = resolvers
