FROM node:12
WORKDIR /usr/src/teal_cache_controller
COPY package*.json ./

RUN npm install

COPY . .

CMD ["node", "index.js"]
