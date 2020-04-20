db.movies.findOne({});

db.movies.find().pretty();

db.movies.find({ name: 'The Last Ship' }).pretty();

db.movies.find({ runtime: 60 }).pretty();

db.movies.find({ runtime: { $eq: 60 } }).pretty();

db.movies.find({ runtime: { $ne: 60 } }).pretty();

db.movies.find({ runtime: { $lt: 42 } }).pretty();

db.movies.find({ runtime: { $lte: 42 } }).pretty();

db.movies.find({ runtime: { $gt: 42 } }).pretty();

db.movies.find({ runtime: { $gte: 42 } }).pretty();

db.movies.find({ 'rating.average': { $gt: 7 } }).pretty();

db.movies.find({ genres: 'Drama' }).pretty();

db.movies.find({ genres: ['Drama'] }).pretty(); // procura específica por array contendo drama

db.movies.find({ runtime: { $in: [30, 42] } }).pretty();

db.movies.find({ runtime: { $nin: [30, 42] } }).pretty();

db.movies.find({ 'rating.average': { $lt: 5 } }).pretty();

db.movies.find({ 'rating.average': { $gt: 9 } }).count();

db.movies
  .find({
    $or: [{ 'rating.average': { $lt: 5 } }, { 'rating.average': { $gt: 9.3 } }],
  })
  .pretty();

db.movies
  .find({
    $nor: [
      { 'rating.average': { $lt: 5 } },
      { 'rating.average': { $gt: 9.3 } },
    ],
  })
  .pretty();

db.movies.find().count();

db.movies
  .find({
    $or: [{ 'rating.average': { $lt: 5 } }, { 'rating.average': { $gt: 9.3 } }],
  })
  .count();

db.movies
  .find({
    $nor: [
      { 'rating.average': { $lt: 5 } },
      { 'rating.average': { $gt: 9.3 } },
    ],
  })
  .count();

db.movies
  .find({
    $and: [{ 'rating.average': { $gt: 9 } }, { genres: 'Drama' }],
  })
  .count();

db.movies.find({ 'rating.average': { $gt: 9 }, genres: 'Drama' }).count();

// db.movies.find({genres: "Drama"}, {"genres":  Horror}).count() // nao pode,
// javascript nao permite ter duas chaves, logo a segunda sobrescreve a primeira

db.movies.find({ $and: [{ genres: 'Drama' }, { genres: 'Horror' }] }).count();

db.movies.find({ runtime: { $not: { $eq: 60 } } }).count();

db.movies.find({ runtime: { $ne: 60 } }).count();

db.movies.find({ summary: 'musical' }).pretty();

db.movies.find({ summary: { $regex: /musical/ } }).pretty();
