# Useful shell commands

## Loops

- prefer piping to `while` rather than `for`
  - `for` loops requires the previous command to finish completely
  - `while` loops process streams

- [zsh differences with other shells](http://zsh.sourceforge.net/FAQ/zshfaq02.html#l10)
- [looping with find](https://stackoverflow.com/a/9612232)

```zsh
# This is whitespace "safe" in zsh since it does not split words by default
fd -tf | while read i; do echo $i; done
find . -type f | while read i; do echo $i; done

# Using xargs executes with word splits, so it needs to be null terminated
# NOTE: xargs is pretty slow compared to using the shell constructs
fd -tf -0 | xargs -0 -n1 -I{} echo "{}"
find . -type f -print0 | xargs -0 -n1 -I{} echo "{}"
```

## In place editing of a file

```bash
# using sponge
cat file | sponge file

# traditional way is to write to a temporary file
cat file > file.bak && mv file.bak file
```

## Why is it so hard to find docs for shell commands?

### echo

- https://ss64.com/bash/echo.html
- http://zsh.sourceforge.net/Doc/Release/Shell-Builtin-Commands.html#index-echo
- `man zshbuiltins` -> search for echo

```bash
# print colours
ANSI_RED="\033[0;31m"
ANSI_RESET="\033[0m"
echo -e "${ANSI_RED}THIS TEXT IS RED${ANSI_RESET}"
```
- https://stackoverflow.com/a/20983251
- https://stackoverflow.com/a/5947802
- https://en.wikipedia.org/wiki/ANSI_escape_code
