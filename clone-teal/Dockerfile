FROM node:12
WORKDIR /usr/src/clone-teal

COPY package*.json ./
RUN npm install

COPY . .

CMD node ./index.js
