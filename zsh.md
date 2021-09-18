# zsh

## Splitting a string to pass as args

In bash using the variable directly works:

```bash
opts="-a -b"
cmd $opts
```

In zsh it needs to be split on spaces (`${=var}` bit), otherwise it passes the var as a single string:

```zsh
opts="-a -b"
cmd ${=opts}
```
