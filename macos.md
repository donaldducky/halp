# MacOS

## Quarantining Apps

When downloading apps from "untrusted sources", MacOS sets a quarantine bit on
the app.

This is sometimes annoying because you'll have to maybe open finder, right
click the binary, select Open. Then select Open again.

This bit can be conveniently removed from the cli.

```bash
# list xattrs
xattr -l /path/to/bin

# remove a particular attr
xattr -d com.apple.quarantine /path/to/bin

# Alternatively, when using homebrew, use the --no-quarantine flag
brew install --no-quarantine <bin>
```
