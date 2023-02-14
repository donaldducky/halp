# Mongo

## mongosh

```sh
# admin is the default auth db
# NOTE: the mongosh process name obfuscates the user/pass
# mongosh mongodb://<credentials>@...
mongosh <database_name> -u <user> -p <password> --authenticationDatabase admin
```

## Basics

```javascript
// dbs
show dbs
use <db>

// collections
show collections

// query
db.<collection>.find()
it

db.<collection>.find({id: 1}).limit(1)
```

## Aggregations

Just putting this here because it's a pain to find the docs.

[Aggregation Quick Reference](https://www.mongodb.com/docs/manual/meta/aggregation-quick-reference/)

```javascript
// example
// $project:
// - mongo always returns _id in the result, we can remove it
// - use the value
// - alias -- similar to "as" in SQL
db.<collection>.aggregate([
  {
    "$match": {
      "type": "my_type",
      "nested.key.id": {
        "$exists": true
      }
    }
  },
  {
    "$project": {
      "_id": 0,
      "type": 1,
      "my_alias": "$nested.value"
    }
  },
  {
    "$limit": 1
  }
])
```
