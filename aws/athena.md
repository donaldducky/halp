# Amazon Athena

Trying to use the AWS Console for this is a terrible experience.

## AthenaCLI

Use [athenacli](https://github.com/dbcli/athenacli)

Note: first use will create `~/.athenacli/athenaclirc`

Minimum to set:
- `region = 'ca-central-1'`
- `role_arn` if you need to assume role
- `work_group = 'primary'`

```bash
# interactive shell (similar to the mysql client)
athenacli

# execute a sql script to produce csv output
athenacli -e script.sql > output.csv
```

## SQL

Supports [Presto functions](https://docs.aws.amazon.com/athena/latest/ug/presto-functions.html).
