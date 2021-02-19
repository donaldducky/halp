# Bash

## redirection

```bash
# write to stderr
>&2 echo "I'm going to stderr"

# redirect stderr
cmd 2> /dev/null

# combine stderr and stdout
cmd > /dev/null 2>&1

# shorter, newer syntax
cmd &> /dev/null
```

## "strict" mode

```bash
#!/bin/bash

set -euo pipefail
```

Short for:
```bash
# exit on error
set -e
# error on unset vars
set -u
# pipeline errors will cause a fail
set -o pipefail
```

Explanation: https://gist.github.com/mohanpedala/1e2ff5661761d3abd0385e8223e16425

## execute until success

```bash
# https://stackoverflow.com/a/24770962
cmd; while [ $? -ne 0 ]; do !!; done
```
