const { Client } = require('pg')

const database_params = {
	host: "api.wjrh.org",
	port: 5432,
	user: "wjrh",
	// database and password are defined in the .pgpass file
}


const Query = {
	schedule: get_schedule
}

const Timeslot = {
	day_of_week: async parent => parent.time_range
} 

async function get_schedule(){

	const client = new Client(database_params)
	await client.connect()
	result = await client.query(`
		SELECT shortname as program_name, time_range
		FROM schedule;
	`)	
	await client.end()

	return result.rows
}


module.exports = {Query, Timeslot}
