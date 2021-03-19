## Display CSV file in table format

From: https://stackoverflow.com/a/3800791

Note: This does not work with values with commas in them.
```bash
column -s, -t < somefile.csv | less -#2 -N -S

# empty values need place holders for column to work
sed 's/,,/, ,/g;s/,,/, ,/g' data.csv | column -s, -t
```

## TODO

- [ ] add csvtool commands
- [ ] look at [csvkit](https://csvkit.readthedocs.io/en/latest/)
