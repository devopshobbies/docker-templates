# Use a small base image
FROM node:14-alpine

# Set the working directory
WORKDIR /opt/src

# Copy only necessary files
COPY package*.json ./
RUN npm install --production && npm cache clean --force

# Copy the rest of the application code
COPY . .

# Use environment variables
ENV PORT=8000
EXPOSE $PORT

# Use a process manager
CMD ["pm2-runtime", "app.js"]
