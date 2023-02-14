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

## [Recursive descent](https://stedolan.github.io/jq/manual/#RecursiveDescent:..)

Yields every value.

Super useful for nested blobs of data.

```bash
echo '{"a":{"b":{"c":27,"c2":"hello"},"b2":[1,2,3]}}' | jq -c ..
{"a":{"b":{"c":27,"c2":"hello"},"b2":[1,2,3]}}
{"b":{"c":27,"c2":"hello"},"b2":[1,2,3]}
{"c":27,"c2":"hello"}
27
"hello"
[1,2,3]
1
2
3
```

```bash
# select values for key "id"
echo '{"id": "outer", "nested": {"id": "inner", "innermost": {"id": {"hi": "there"}}}}' | jq -cr '.. | .id? | select(.)'
outer
inner
{"hi":"there"}
```

```bash
# select all objects with key "id"
echo '{"id": "outer", "nested": {"id": "inner", "innermost": {"id": {"hi": "there"}}}, "array": [{"id": 1, "value": "one"},{"id": 2, "value": "two"}]}' | jq -c '..|select(has("id"))?'
{"id":"outer","nested":{"id":"inner","innermost":{"id":{"hi":"there"}}},"array":[{"id":1,"value":"one"},{"id":2,"value":"two"}]}
{"id":"inner","innermost":{"id":{"hi":"there"}}}
{"id":{"hi":"there"}}
{"id":1,"value":"one"}
{"id":2,"value":"two"}
```
