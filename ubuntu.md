# Ubuntu

## Adding a new apt repo

```bash
# required for `add-apt-repository`
sudo apt-get install software-properties-common
sudo add-apt-repository ppa:ondrej/php
```

## Listing available versions

```bash
sudo apt-get update
apt-cache policy <pkg>
```
