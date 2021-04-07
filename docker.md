# Docker

## Getting root access into a running container

Containers should run as an unprivileged user as a best practice.
Sometimes, you may need to do something as `root`, such as editing config files
or creating folders and it may be more cumbersome rebuilding the container.

```bash
# get a bash shell as root, user=0
docker exec -it -u 0 $(docker ps --filter=name=my_container -q) bash
```

## Editing files on a stopped container

You may need to do this if you edit a config file (ie. `httpd.conf`) and
restart the service but the config file is bad so it crashes the service.

```bash
docker cp <container_name>:<path-to-file> <local-file>

# Example copying /etc/httpd/conf/httpd.conf from container my_web_1
docker cp my_web_1:/etc/httpd/conf/httpd.conf httpd.conf
# edit file and put it back
docker cp httpd.conf my_web_1:/etc/httpd/conf/httpd.conf
```
