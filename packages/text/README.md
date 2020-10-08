# Text

## Setup / Running

1. `npm ci`
1. `npm start`

Translation data is stored in `./data`. If the data already exists for a
particular translation then building it is skipped by this script. To re-build
a translation, delete the associated data and re-run the script.

## Downloading Pre-Existing Data

I recommend downloading the data for the NET translation if you are able to,
rather than building from scratch, because that build takes the longest (it
uses their public API server and I throttled the script so it wouldn't hit
their servers too hard).

```sh
wget -P data https://f001.backblazeb2.com/file/instant-bible/BSB.pb
wget -P data https://f001.backblazeb2.com/file/instant-bible/NET.pb
wget -P data https://f001.backblazeb2.com/file/instant-bible/KJV.pb
```
