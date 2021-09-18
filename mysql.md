# MySQL

## Working with JSON

https://dev.mysql.com/doc/refman/8.0/en/json-function-reference.html#docs-body

```sql
-- select .field
SELECT JSON_EXTRACT(data, '$.field') FROM <table>;
SELECT JSON_EXTRACT(data, '$.field.nested') FROM <table>;

-- set .field
UPDATE <table> SET JSON_SET(data, '$.field', '<new-val>');

-- pretty
SELECT JSON_PRETTY(data) FROM <table>;
```

## Who Am I?

Depending on the context, `CURRENT_USER` and `USER` can be different.
See: https://dev.mysql.com/doc/refman/8.0/en/information-functions.html#function_current-user

```sql
SELECT CURRENT_USER();
SELECT USER();
```
