#!/usr/bin/node
// Copies metaredis from every show and episode in the teal
// redisbase into a local redis cache, which can be used for
// much faster read times
//
// This script will take a couple minutes to run. It is recommended
// to run it as a cron-job scheduled once every couple of hours

const node_fetch = require('node-fetch')
const ioredis = require('ioredis')
const redis = new ioredis(6379, 'localhost')

// the main script, encapsulated in a function so we can make
// asynchronous calls
async function run(){

	console.log('Updating Teal Cache')
	console.time('update_cache')
	console.log('pulling redis from teal...')

	const programs = await fetch(`https://api.teal.cool/organizations/wjrh`)
	const program_names = programs.map(program => program.shortname)
	redis.sadd('programs', ...program_names)

	for (const name of program_names){
		console.log(`Pulling data for program: ${name}`)
		console.time(`${name}`)

		const program = await fetch(`https://api.teal.cool/programs/${name}`)
		const {owners, tags, episodes, ...scalars} = program

		// write the "scalar" metadata into a hashmap 
		write_hashmap(`programs:${name}`, scalars)

		// because redis does not support nested data structures,
		// the "vector" metadata, i.e. arrays like tags and episodes,
		// needs to be treated differently

		if(tags && tags.length >= 1){
			// tags are stored as a set
			tags.forEach(tag => redis.sadd(`tags:${tag}`, name))
		}

		// Check to see if this program has any episodes
		if(episodes && episodes.length >= 1){
			// episodes are stored as a separate hashmap with key equal to the 
			// episode id, and a set of ids linking each program to its episodes
			episode_ids = program.episodes.map(episode => episode.id)
			program.episodes.forEach(episode => write_hashmap(`episodes:${episode.id}`, episode))
			redis.sadd(`programs:${name}:episodes`, ...episode_ids)
		}

		console.timeEnd(`${name}`)
		console.log()
	}

	console.timeEnd('update_cache')
	console.log('finished')
}



// ----------------
// helper functions
// ----------------


// writes a "flat" object into a redis hashmap
// the hmset command takes arguments in the order:
// hashmap-key, key1, value1, key2, value2, ...
const write_hashmap = function(key, object){
	const entries = Object.entries(object)
		.filter(([key, value]) => value != null)
		.flat()
		.map(val => `${val}`)
	redis.hmset(`${key}:scalars`, ...entries)
}

// Wrapper for the fetch function that automatically
// converts to JSON and handles errors
const fetch = async function(url){
	const result = await node_fetch(url)
		.then(res => res.json())
		.catch(err => {
			console.log(err)
			return {}
		})

	return result
}

run()
