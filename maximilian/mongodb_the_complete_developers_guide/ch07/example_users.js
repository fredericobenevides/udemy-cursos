use user

db.users.insertMany([
  {name: "Max", hobbies: [{title: "Sports", frequency: 3}, {title: "Cooking", frequency: 6}], phone: 0123312},
  {name: "Manuel", hobbies: [{title: "Cars", frequency: 2}, {title: "Cooking", frequency: 5}], phone: "1213132", age: 30}
])

db.users.find().pretty()


db.users.find({age: {$exists: true}}).pretty()

db.users.find({age: {$exists: true, $gt: 30}}).pretty()

db.users.find({age: {$exists: true, $gte: 30}}).pretty()

db.users.insertOne(
  {name: "Anna", hobbies: [{title: "Sports", frequency: 2}, {title: "Yoga", frequency: 3}], phone: "1231321", age: null},
)

db.users.find({age: {$exists: true, $ne: null}}).pretty()

db.users.find({phone: {$type: "number"}}).pretty()

db.users.find({phone: {$type: "double"}}).pretty()

db.users.find({phone: {$type: ["double", "string"]}}).pretty()



db.users.find().pretty()

db.users.find({"hobbies.title": "Sports"}).pretty()



db.users.insertOne({name: "Chris", hobbies: ["Sports", "Cooking", "Hiking"]})

db.users.find({hobbies: {$size: 3}}).pretty()

