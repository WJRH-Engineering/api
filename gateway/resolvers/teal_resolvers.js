const fetch = require('node-fetch')
const redis = require('ioredis')
const data = new redis(6379, 'api.wjrh.org')

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

module.exports = {Query, Program, Episode}
