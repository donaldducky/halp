# Yum

Yum package manager.

Used in CentOS.

## Commands

Cheatsheet:
https://access.redhat.com/sites/default/files/attachments/rh_yum_cheatsheet_1214_jcs_print-1.pdf

https://docs.oracle.com/cd/E37670_01/E37355/html/ol_creating_yum_repo.html

```bash
# list available packages
yum list

# list installed packages
yum list installed

# list repos
yum repolist

# install repo
yum install https://dl.fedoraproject.org/pub/epel/epel-release-latest-7.noarch.rpm -y

# alternative install using rpm
wget https://dl.fedoraproject.org/pub/epel/epel-release-latest-7.noarch.rpm
rpm -Uvh epel-release-latest-7.noarch.rpm

# to remove (install with path to rpm but remove with the name)
yum remove epel-release

# see what package gives a specific binary
yum provides ntpdate

# list package versions
yum --showduplicates list <pkg-name>

# downgrade a version
yum downgrade <pkg>-<version>

# view dependencies
yum deplist <pkg-name>

# see what depends on a package
rpm -q --whatrequires sqlite
```
