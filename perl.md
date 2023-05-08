# perl

Useful for quick one-liners from the terminal.

More consistent than `sed` across platforms and more convenient "perl" regex.

## Regex replace

```bash
echo "hello there" | perl -pe 's/there/world/'
hello world

# semi-colon required to separate commands
echo "hello there" | perl -pe 's/there/world/;' -pe 's/hello/goodbye/'
goodbye world
```
