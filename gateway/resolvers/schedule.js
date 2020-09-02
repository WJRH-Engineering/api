const { Client } = require('pg')

const database_params = {
	host: "database",
	port: 5432,
	user: "wjrh",
	database: "testdb",
	password: "hogghall"
	// database and password are defined in the .pgpass file
}

const schema = `
extend type Query {
	schedule: [Timeslot]
}

extend type Mutation {
	add_timeslot(input: TimeslotInput): [Timeslot]
	delete_timeslot(id: Int): String
	update_timeslot(id: Int, input: TimeslotInput): String
}

input TimeslotInput {
	shortname: String
	start_time: String
	end_time: String
	day: Int
	end_day: Int # ignore if show does not span multiple days
	year: Int
	season: String
}

extend type Timeslot{
	id: Int
	shortname: String
	time_range: String
	day_of_week: String
	day_number: Int
	start_hour: Int
	end_hour: Int

	# this edge defined in resolvers/teal.js
	# program: Program
}`

const Query = {
	schedule: get_schedule
}
const Mutation = {
	add_timeslot: (parent, args) => add_timeslot(args.input),
	delete_timeslot: (parent, args) => delete_timeslot(args.id),
	update_timeslot: (parent, args) => update_timeslot(args.id, args.input)
}
const weekdays = ["", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"]

const Timeslot = {
	day_of_week: parent => weekdays[parent.day_number]
} 


async function update_timeslot(id, args){
	const client = new Client(database_params)
	await client.connect()

	console.log(args)

	const {start_time, end_time, day, end_day, ...info} = args

	if (start_time || end_time || day || end_day){
		info.time_range = `[1996-01-${day} ${start_time}:00, 1996-01-${end_day || day} ${end_time}:00)`
	}		

	Object.entries(info).forEach(async function([key, value]){
		await client.query(`
			UPDATE schedule	SET ${key} = $1
		;`,[value])	
	})
	return "success"
	
}

async function add_timeslot(args){
	const {shortname, start_time, end_time, day, end_day} = args
	const client = new Client(database_params)
	await client.connect()

	// the sql database wants the timerange string in a specific format
	const range_string = `[1996-01-${day} ${start_time}:00, 1996-01-${end_day || day} ${end_time}:00)`
	
	result = await client.query(`
		INSERT INTO schedule
			(shortname, time_range)
		VALUES
			($1, $2)
		;
	`, [shortname, range_string])

	return await get_schedule()
}

async function delete_timeslot(id){
	const client = new Client(database_params)
	await client.connect()
	result = await client.query(`
		DELETE FROM schedule WHERE id = $1;
	`, [id])	
	await client.end()
	
	return "success"
}

async function get_schedule(){

	const client = new Client(database_params)
	await client.connect()
	result = await client.query(`
		SELECT 
			id,
			shortname as shortname,
			time_range,
			EXTRACT(isodow FROM lower(time_range)) as day_number,
			EXTRACT(hour FROM lower(time_range)) as start_hour,
			EXTRACT(hour FROM upper(time_range)) as end_hour

		FROM schedule;
	`)	
	await client.end()

	return result.rows
}

module.exports = {schema, resolvers:{Mutation, Query, Timeslot}}
