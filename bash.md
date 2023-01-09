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

## heredocs

- https://www.gnu.org/savannah-checkouts/gnu/bash/manual/bash.html#Here-Documents

```bash
# Using quotes around "word", EOF in this case, prevents expansion
cat <<"EOF"
My output here
EOF

# Without quotes allows expansion
var="hello there"; cat <<EOF
My output with $var expanded
EOF

# Using a "-" strips trailing tabs allowing for indenting content.
	cat <<-"EOF"
	Hello there
EOF
```

## parameter expansion

- https://www.gnu.org/savannah-checkouts/gnu/bash/manual/bash.html#Shell-Parameter-Expansion

```bash
# expand $1 or set default if null or unset
my_var=${1:-default}
```

## Modulino

Useful when you want a script to be
- runnable; and
- imported to be used like a library

See: [Modulinos in Bash](https://blog.dnmfarrell.com/post/modulinos-in-bash/)

Example modulino:
```bash
#!/bin/bash

function hello {
  if [[ -n "$1" ]];then
    name="$1"
  else
    name="World"
  fi
  echo "Hello, $name!"
}
[[ "$BASH_SOURCE" == "$0" ]] && hello "$@"
```

Example test script using the modulino (`source "./hello.bash"`):
```bash
#!/bin/bash

PASS=0

function fail {
  echo "$1"
  PASS=1
}

source "./hello.bash"

def=$(hello)
[[ "$def" == "Hello, World!" ]] || fail "wrong default greeting: $def"

arg=$(hello David)
[[ "$arg" == "Hello, David!" ]] || fail "wrong arg greeting: $arg"

exit "$PASS"
```
