const { Client } = require('pg')

const database_params = {
	host: "api.wjrh.org",
	port: 5432,
	user: "wjrh",
	// database and password are defined in the .pgpass file
}

const schema = `
extend type Query {
	schedule: [Timeslot]
}

extend type Timeslot{
	shortname: String
	time_range: String
	day_of_week: String
	day_number: Int

	# this edge defined in resolvers/teal.js
	# program: Program
}`

const Query = {
	schedule: get_schedule
}
const weekdays = ["", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"]

const Timeslot = {
	day_of_week: parent => weekdays[parent.day_number]
} 

async function get_schedule(){

	const client = new Client(database_params)
	await client.connect()
	result = await client.query(`
		SELECT 
			shortname as shortname,
			time_range,
			EXTRACT(isodow FROM lower(time_range)) as day_number
		FROM schedule;
	`)	
	await client.end()

	return result.rows
}


module.exports = {schema, resolvers:{Query, Timeslot}}
