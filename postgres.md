# Postgres

## JSON

### Select

https://www.postgresql.org/docs/12/functions-json.html

```sql
-- pretty print
select jsonb_pretty(my_json_field) from my_table;

-- select as json/jsonb
select
  my_json_array->2,      -- array element
  my_json_object->'key', -- object value
  my_json#>'{a,b}'       -- object by path
from my_table;

-- select as text
select
  my_json_array->>2,
  my_json_object->>'key',
  my_json#>>'{a,b}'
from my_table;
```

[More operators](https://www.postgresql.org/docs/12/functions-json.html#FUNCTIONS-JSONB-OP-TABLE)

```sql
-- jsonpath
select jsonb_pretty(payload) from my_table where payload @@ '$.a.b == "some_value"' limit 10
select count(*) from my_table where payload @? '$.content.sessionId';
select count(*) from my_table where not payload @? '$.content.sessionId';
```

There is a lot more to look at:
- [ ] [creation functions](https://www.postgresql.org/docs/12/functions-json.html#FUNCTIONS-JSON-CREATION-TABLE)
- [ ] [processing functions](https://www.postgresql.org/docs/12/functions-json.html#FUNCTIONS-JSON-PROCESSING-TABLE)
- [ ] [path language](https://www.postgresql.org/docs/12/functions-json.html#FUNCTIONS-SQLJSON-PATH) to use with `@@` and `@?`
- [ ] [jsonb_agg](https://www.postgresql.org/docs/12/functions-aggregate.html#id-1.5.8.25.4.2.2.12.1.1)
  - useful to return json back from the db rather than a traditional join and post-processing
  - https://medium.com/@geekuillaume/sql-tip-jsonb-agg-in-postgresql-for-simple-one-to-many-joins-bde8caa30c46

### json vs. jsonb

Docs: https://www.postgresql.org/docs/current/datatype-json.html

Ideally prefer `jsonb` for efficiency reasons.

`json`
- stores exact copy -> requires parsing on every use
- data:
  - preserves whitespace
  - preserves object key ordering
  - preserves duplicate object keys

`jsonb`
- stores decomposed binary -> small overhead on insert
- supports indexing
- data:
  - last object key is stored (discarding duplicates)
- beware:
  - limitations of jsonb vs. json
    - unicode escapes must be valid (ie. \u0000 cannot be represented by pg)
      - jsonb folds escapes into its single character unicode representation
    - numbers greater than pg's `numeric` data type will error (but won't if it's `json`)

JSON types -> pg types [doc](https://www.postgresql.org/docs/current/datatype-json.html#JSON-TYPE-MAPPING-TABLE)

```
string  text    \u0000 is disallowed, as are Unicode escapes representing characters not available in the database encoding
number  numeric NaN and infinity values are disallowed
boolean boolean Only lowercase true and false spellings are accepted
null    (none)  SQL NULL is a different concept
```

### Containment and existence

https://www.postgresql.org/docs/current/datatype-json.html#JSON-CONTAINMENT

```sql
-- containment
SELECT '{"product": "PostgreSQL", "version": 9.4, "jsonb": true}'::jsonb @> '{"version": 9.4}'::jsonb;

-- existence (works on arrays and object keys)
SELECT '{"foo": "bar"}'::jsonb ? 'foo';
```

### Indexing

https://www.postgresql.org/docs/current/datatype-json.html#JSON-INDEXING

```sql
-- default GIN jsonb_ops
CREATE INDEX idxgin ON api USING GIN (jdoc);

-- Find documents in which the key "company" has value "Magnafone"
SELECT jdoc->'guid', jdoc->'name' FROM api WHERE jdoc @> '{"company": "Magnafone"}';

-- GIN jsonb_path_ops
CREATE INDEX idxginp ON api USING GIN (jdoc jsonb_path_ops);

-- Find documents in which the key "tags" contains key or array element "qui"
SELECT jdoc->'guid', jdoc->'name' FROM api WHERE jdoc -> 'tags' ? 'qui';

-- expression index
CREATE INDEX idxgintags ON api USING GIN ((jdoc -> 'tags'));
```

GIN
- stores keys and values
- containment queries are ok
- cannot search by `?` operator
- supports [jsonpath](https://www.postgresql.org/docs/current/datatype-json.html#DATATYPE-JSONPATH) matching using `@@` and `@?`

jsonb_path_ops
- stores values
- does not support empty queries -> causes index scan
- index size is smaller than GIN
- queries are more targeted and typically faster than GIN (but less flexible)

expression indexes
- for targeted queries

## MySQL "show create table" equivalent

```bash
PGPASSWORD='postgres' -h localhost -U postgres pg_dump -st my_table my_db
```

## MySQL \G equivalent

```sql
\x
Expanded display is on.

-- run queries

\x
Expanded display is off.
```

## postgresql homebrew

```bash
psql -U postgres
```

> FATAL: role "postgres" does not exist

```bash
# on m1: /opt/homebrew
# on intel: /usr/local
$(brew --prefix)/opt/postgres/bin/createuser -s postgres
```
