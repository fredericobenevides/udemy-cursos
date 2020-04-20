use contactData

db.contacts.findOne()

db.contacts.find({"dob.age": {$gt: 60}}).pretty()

db.contacts.find({"dob.age": {$gt: 60}}).count()

db.contacts.explain().find({"dob.age": {$gt: 60}})

db.contacts.explain("executionStats").find({"dob.age": {$gt: 60}}) // docs examine 5000 returned 1222

db.contacts.createIndex({"dob.age": 1})

db.contacts.explain("executionStats").find({"dob.age": {$gt: 60}})

db.contacts.explain("executionStats").find({"dob.age": {$gt: 20}})

db.contacts.dropIndex({"dob.age": 1})

db.contacts.createIndex({gender: 1})

db.contacts.dropIndex({gender: 1})

db.contacts.explain("executionStats").find({gender: "male"})

db.contacts.createIndex({"dob.age": 1, gender: 1})

db.contacts.explain("executionStats").find({"dob.age": 35, gender: "male" })
