## Display CSV file in table format

From: https://stackoverflow.com/a/3800791

Note: This does not work with values with commas in them.
```bash
column -s, -t < somefile.csv | less -#2 -N -S

# empty values need place holders for column to work
sed 's/,,/, ,/g;s/,,/, ,/g' data.csv | column -s, -t

# column outputs tabs, spaces can be used via expand
sed 's/,,/, ,/g;s/,,/, ,/g' data.csv | column -s, -t | expand
```

## csv to JSON

```zsh
# csvjson
brew install csvkit
# or open new terminal
hash -r
```

```zsh
cat some.csv | csvjson 
```

## TODO

- [ ] add csvtool commands
- [ ] look at [csvkit](https://csvkit.readthedocs.io/en/latest/)
