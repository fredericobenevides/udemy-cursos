use flights

show dbs

db.flightData.insertOne({
    "departureAirport": "MUC",
    "arrivalAirport": "SFO",
    "aircraft": "Airbus A380",
    "distance": 12000,
    "intercontinental": true
})

db.flightData.insertOne({departureAirport: "TXL", arrivelAirport: "LHR"})

db.flightData.insertOne({departureAirport: "TXL", arrivelAirport: "LHR", _id: "txl-lhr-1"})

db.flightData.find().pretty()

db.flightData.deleteOne({departureAirport: "TXL"})

db.flightData.updateOne({distance: 12000}, {$set: {marker: 'delete'}})

db.flightData.updateMany({}, {$set: {marker: 'toDelete'}})

db.flightData.deleteMany({marker: "toDelete"})

db.flightData.insertMany([
  {
    "departureAirport": "MUC",
    "arrivalAirport": "SFO",
    "aircraft": "Airbus A380",
    "distance": 12000,
    "intercontinental": true
  },
  {
    "departureAirport": "LHR",
    "arrivalAirport": "TXL",
    "aircraft": "Airbus A320",
    "distance": 950,
    "intercontinental": false
  }
])

db.flightData.find({intercontinental: true}).pretty()

db.flightData.find({distance: 12000}).pretty()

db.flightData.find({distance: {$gt: 1000}}).pretty()

db.flightData.find({distance: {$gt: 900}}).pretty()

db.flightData.findOne({distance: {$gt: 900}})

db.flightData.updateOne({_id:ObjectId("5e93c8fe8f1af2a035878cf9")}, {$set: {delayed: true}})

db.flightData.update({_id:ObjectId("5e93c8fe8f1af2a035878cf9")}, {$set: {delayed: false}})

db.flightData.update({_id:ObjectId("5e93c8fe8f1af2a035878cf9")}, {delayed: false})

db.flightData.find().pretty()

db.flightData.replaceOne({_id:ObjectId("5e93c8fe8f1af2a035878cf9")}, {
    "departureAirport": "MUC",
    "arrivalAirport": "SFO",
    "aircraft": "Airbus A380",
    "distance": 12000,
    "intercontinental": true
})

db.flightData.updateMany({}, {$set: {status: {description: "on-time", lastUpdated: "1 hour ago"}}})

db.flightData.updateMany({}, {$set: {status: {description: "on-time", lastUpdated: "1 hour ago", details: { responsible: 'Frederico Benevides'}}}})

db.flightData.find().pretty()
