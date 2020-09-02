// index.js
const { ApolloServer, makeExecutableSchema, gql } = require('apollo-server')

const fs = require('fs')

// const typeDefs = require('./schema.js')
// const main_resolvers = require('./resolvers.js')

// define an empty Query type here so that it can 
// be extended later
const base_types = `
type Query {
	_blank: String
}
type Mutation {
	_blank: String
}
type Timeslot {
	_blank: String
}`

let graphql_data = []
graphql_data.push(require('./resolvers/teal.js'))
graphql_data.push(require('./resolvers/schedule.js'))
graphql_data.push(require('./resolvers/lastfm.js'))

// combine all schema strings into a single string
const schema = graphql_data
	.map(data => data.schema || '')
	.reduce((prev, cur) => prev + cur, base_types)

// combine all resolver objects
const resolvers = graphql_data
	.map(data => data.resolvers || [])

const server = new ApolloServer({ 
	cors: true,
	typeDefs: gql(schema),
	resolvers: resolvers,
})

server.listen()
	.then(({url}) => console.log(`server ready at ${url}`))
