# paste

```bash
# use paste with stdin
cat file | paste -sd',' -

# join columns
paste file1 file2

# using process substitution, useful when processing files with pipelines
paste <(cat file1 | ...) <(cat file2 | ...)

# join columns with space delimiter
paste -d' ' file1 file2

# join all lines into one with a delimiter
paste -sd',' file
```
