FROM node:14-alpine

WORKDIR /usr/src/app

COPY rollup.config.js ./
COPY svelte.config.js ./
COPY tsconfig.json ./
COPY package*.json ./

RUN npm install

COPY ./src ./src
COPY ./public ./public

ARG BOOKS_HOST
ARG BOOKS_PORT
RUN BOOKS_HOST=$BOOKS_HOST BOOKS_PORT=$BOOKS_PORT npm run build

ENV HOST=0.0.0.0

CMD [ "npm", "start" ]
