const {oneLineTrim} = require('common-tags')
const LASTFM_API_KEY = "14cacc2d28210dcd318ffa2085778844"

// const Query = {lastfm_data: function(parent)}
const Track = {
	lastfm_data: fetch_lastfm_data
} 

async function fetch_lastfm_data({artist, title}){
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

module.exports = {Track}
