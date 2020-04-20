use contactData

db.persons.insertOne({name: "Max", age: 30, hobbies: ["Sports", "Cooking"]})

db.persons.insertOne({name: "Manuel", age: 31, hobbies: ["Cars", "Cooking"]})

db.persons.insertMany([
  {name: "Anna", age: 29, hobbies: ["Sports", "Yoga"]},
])

db.persons.insertMany([
  {name: "Maria", age: 31},
  {name: "Chris", age: 25},
])

db.persons.insert({name: "Phil", age: 35})

db.persons.insert([{name: "Sandeep", age: 28}, {name: "Hans", age: 38}])



db.persons.insertOne({name: "Max", age: 30}, {writeConcern: {w: 0}})

db.persons.insertOne({name: "Max", age: 30}, {writeConcern: {w: 1, j: true}})

db.persons.insertOne({name: "Max", age: 30}, {writeConcern: {w: 1, j: true, wtimeout: 1}})
