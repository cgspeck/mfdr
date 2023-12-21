# MyGPO Feed Downloader Runner

Utility to periodically scan a self-hosted Gpodder.net web-service database and run the feed downloader script.

Needs postgres libraries to build:

```shell
sudo pacman -S postgresql-libs
```

Uses the same `DATABASE_URL` environment variable that the Gpodder service uses.

Usage:

```
./mfdr --help
MyGPO Feed Downloader Runner

Usage: mfdr [OPTIONS]

Options:
  -d, --daemon
  -s, --sleep-interval <MINUTES>  [default: 480]
  -h, --help                      Print help
  -V, --version                   Print version
```
