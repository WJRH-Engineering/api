#!/usr/bin/node

const fetch = require('node-fetch')
const redis = require('ioredis')
const data = new redis(6379, 'redis')

data.on('error', console.error)

const teal_request = async function(url){
	const request = fetch(url)
		.then(res => res.json())
		.catch(err => {})
		
	const result = await request
	return result
}

const store_program = function(program){
	const id = program.shortname
	const {owners, tags, episodes, ...scalars} = program

	// add program id to global set
	data.sadd('programs', id)
	
	// store all scalar properties in a redis hashmap
	const entries = Object.entries(scalars)
		.filter(([key, value]) => value != null)
		.flat()
		.map(val => `${val}`)
	data.hmset(`program:${id}:scalars`, ...entries)
	
	if(episodes != null && episodes != []){
		// store episode ids in a redis set
		const episode_ids = episodes.map(episode => episode.id)
		data.sadd(`program:${id}:episodes`, episode_ids)

		// store each episode in redis hashmap
		episodes.forEach(store_episode)
	}
}

const store_episode = async function(episode){
	const id = episode.id
	const { tracks, ...scalars } = episode
	
	// store all scalar properties in a redis hashmap
	const entries = Object.entries(scalars)
		.filter(([key, value]) => value != null)
		.flat()
		.map(val => `${val}`)
	data.hmset(`episode:${id}:scalars`, ...entries)

	// fetch tracks
	//const tracks = await fetch()	
}

const get_organization = async function(){
	const url = `https://api.teal.cool/organizations/wjrh`
	const result = await teal_request(url)
	result.forEach(store_program)
	return result
}

const get_program = async function(shortname){
	const url = `https://api.teal.cool/programs/${shortname}`
	const program = await teal_request(url)
	store_program(program)
	return program
}

const update_tealcache = async function(){
//	console.log('Updating Teal Cache')
//	console.time('update_cache')
//	console.log('pulling data from teal...')

	const programs = await get_organization()

	//programs.forEach(program => console.log(program.shortname))
	
	for (const shortname of programs.map(program => program.shortname)){
	//	console.log(shortname)
//		console.time(shortname)
		const program = await get_program(shortname)
		
//		for(episode of program.episodes){
//			console.log(episode.id)
//			const request = await fetch(`https://api.teal.cool/episodes/${episode.id}`)
//			const result = await request.json()
//			
//
//			if(result.tracks != null && result.tracks != []){
//				result.tracks.forEach(function(track){
//					data.sadd(`episode:${episode.id}:tracks`, track.id)
//					const scalars = Object.entries(track)
//						.filter(([key, value]) => value != null)
//						.flat()
//					data.hmset(`track:${track.id}:scalars`, scalars)
//				})
//			}
//	

//		console.timeEnd(`${program.shortname}`)
	}

	//console.timeEnd('update_cache')
	//console.log('finished')
}

const update = async function(){
	data.flushall()
	const programs = await data.smembers('programs')
	console.log(programs)
	if(programs.length == 0){
		update_tealcache()
	}

	setTimeout(update, 60 * 60 * 1000)
}

update()

// run update every hour
setTimeout(update, 60 * 60 * 1000)
