# ripgrep

https://github.com/BurntSushi/ripgrep

```bash
# exclude directory
rg findme --glob '!but-not-here'
rg findme -g '!but-not-here'

# exclude files containing match
rg pattern --files-without-match

# multiline
rg --multiline pattern
rg -U pattern
# to have dot match newlines
rg --multiline '(?s)a.*b'
rg --multiline-dotall 'a.*b'
```
