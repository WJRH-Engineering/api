// index.js
const { ApolloServer, makeExecutableSchema, gql } = require('apollo-server')

const fs = require('fs')

const typeDefs = require('./schema.js')
const resolvers = require('./resolvers.js')

const server = new ApolloServer({ typeDefs, resolvers })
server.listen().then(({url}) => console.log(`server ready at ${url}`))
