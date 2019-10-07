require('./models/User')
require('./models/Track')

const express = require('express')
const mongoose = require('mongoose')
const bodyParser = require('body-parser')
const authRoutes = require('./routes/authRoutes')
const trackRoutes = require('./routes/trackRoutes')
const requireAuth = require('./middlewares/requireAuth')

const app = express();

app.use(bodyParser.json())
app.use(authRoutes)
app.use(trackRoutes)

const mongoUri = 'mongodb://localhost:27017/myapp'
// const mongoUri = 'mongodb://admin:admin@localhost:27017/myapp?authSource=myapp&w=1'
mongoose.connect(mongoUri, {
  // authdb: "admin",
  // user: "admin",
  // password: "admin",
  useNewUrlParser: true,
  useCreateIndex: true
})
mongoose.connection.on('connected', () => {
  console.log('Connected to mongo instance')
})
mongoose.connection.on('error', (err) => {
  console.error('Error connection to mongo', err)
})

app.get('/', requireAuth, (req, res) => {
  res.send(`Your email: ${req.user.email}`)
  // res.send('Hi there!')
})

app.listen(3000, () => {
  console.log('Listening on port 3000');
})
