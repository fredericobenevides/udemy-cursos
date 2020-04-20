use support

db.questionThreads.insertOne({creator: "Max", question: "How does that all work?", answers: ["q1a1", "q1a2"]})

db.questionThreads.findOne()

db.answerThreads.insertMany([{_id: "q1a1", text: "It works like that."}, {_id:"q1a2", text: "Thanks!"}])

db.answerThreads.find().pretty()


db.questionThreads.deleteMany({})

db.questionThreads.insertOne({creator: "Max", question: "How does that all work?", answers: [{ text: "Like that."}, {text: "Thanks!"}]})

db.questionThreads.find().pretty()
