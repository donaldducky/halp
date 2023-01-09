# jq

## Working with non-json lines

Useful for parsing logs containing structured (json) or unstructured data.

```bash
# filter non-json lines
# https://github.com/stedolan/jq/issues/884#issuecomment-128439361
output | jq -R 'fromjson? | select(type == "object")'

# parse json and print non-json lines
# https://github.com/stedolan/jq/issues/1547#issuecomment-406596374
output | jq -Rrc '. as $line | try (fromjson | .) catch $line'
```

## Set exit code on falsy

```bash
# Test and exit non-zero when falsy
output | jq -e '.something == 1'
```

## Combining data

```bash
# use -s to stream multiple inputs
jq -s '.|flatten' <(echo '[[{"id":1},{"id":2}]]') <(echo '[[{"id":3},{"id":4}]]')
[
  {
    "id": 1
  },
  {
    "id": 2
  },
  {
    "id": 3
  },
  {
    "id": 4
  }
]

# map to entries (ie. like to_entries) and use from_entries
# in JSON, the object key must be a string
jq -s '.|flatten|map({key: "\(.id)", value: .})|from_entries' <(echo '[{"id":1,"name":"a"},{"id":2,"name":"b"}]') <(echo '[{"id":3,"name":"c"},{"id":4,"name":"d"}]')
{
  "1": {
    "id": 1,
    "name": "a"
  },
  "2": {
    "id": 2,
    "name": "b"
  },
  "3": {
    "id": 3,
    "name": "c"
  },
  "4": {
    "id": 4,
    "name": "d"
  }
}
```
