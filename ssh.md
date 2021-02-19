# ssh

## cat file to remote server

```bash
cat << EOF | ssh user@host "cat > remote-file"
Hello, I'm going to be stored in remote-file for user@host
EOF
```

## background a process

```bash
ssh -f user@host "nohup cmd > /dev/null 2>&1 &"

# there may be an issue with ssh hanging, in which case redirecting all three I/O streams should fix it
# https://en.wikipedia.org/wiki/Nohup#Overcoming_hanging
nohup ./myprogram > foo.out 2> foo.err < /dev/null &
```
