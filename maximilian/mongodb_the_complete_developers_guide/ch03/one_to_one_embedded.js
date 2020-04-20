use hospital

db.patients.insertOne({name: "Max", age:29, diseaseSummary: 'summary-max-1'})

db.patients.findOne()

db.diseaseSummaries.insertOne({_id: "summary-max-1", diseases: ["cod", "broken leg"]})

db.diseaseSummaries.findOne()


db.patients.findOne().diseaseSummary

var dsid = db.patients.findOne().diseaseSummary
db.diseaseSummaries.findOne({_id: dsid})



db.patients.deleteMany({})

db.patients.insertOne({name: "Max", age:29, diseaseSummary: {diseases: ["cold", "broken leg"]}})

db.patients.findOne()


