# Crawler

## Quick Start

Note that this will download 241.1 MB of gzipped data which will expand to 1.3
GB on disk.

```sh
curl https://f001.backblazeb2.com/file/instant-bible/2020-05.txt.gz | gunzip > data/2020-05.txt
```

## Super Long Start

I seriously do not recommend doing this unless you have tons of disposable
bandwidth and processing power at your disposal. Running this script with the
given parameters will download over 10 TB of data. On a 32-core machine this
script took five hours to run. You have been warned.

In order for this script to run you will need
[ripgrep](https://github.com/BurntSushi/ripgrep) installed.

```sh
./crawl.sh 2020 05
./collate.sh 2020 05
```
