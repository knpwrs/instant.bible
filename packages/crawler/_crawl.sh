#!/usr/bin/env bash

curl -s --retry 5 $1/$2 | zcat | rg -io -a -f verse.regex.build > $3/$(basename $2 .warc.wet.gz)
