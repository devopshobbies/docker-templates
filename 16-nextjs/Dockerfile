FROM node:16-alpine as runtime

WORKDIR /app

COPY package*.json ./

RUN npm install --only-production

COPY . .

RUN npm run build

EXPOSE 3000

CMD ["npm", "start"]