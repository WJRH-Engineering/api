const fetch = require('node-fetch')
const redis = require('ioredis')
const data = new redis(6379, 'api.wjrh.org')

const teal = require('./resolvers/teal_resolvers.js')
const lastfm = require('./resolvers/lastfm_resolvers.js')
const schedule = require('./resolvers/schedule_resolvers.js')

const resolvers = {
	Query: {...teal.Query, ...schedule.Query},
	Program: teal.Program,
	Episode: teal.Episode,
	Track: lastfm.Track,
	Timeslot: schedule.Timeslot,
}

module.exports = resolvers
