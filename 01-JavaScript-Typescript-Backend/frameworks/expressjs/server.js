const express = require('express')
const path = require('path')

const app = express()

// setup static and middleware
app.use(express.static('./public'))
app.use(express.static('./public/errorPage'))

app.get('/', (req, res) => {
  res.sendFile(path.resolve(__dirname, './index.html'))
})

app.all('*', (req, res) => {
    res.status(404).sendFile(path.resolve(__dirname, './public/errorPage/error.html'))
})

app.listen(8000, () => {
  console.log('server is listening on port 8000....')
})
