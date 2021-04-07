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
