#!/usr/bin/env bash

YEAR="$1"
CRAWL="$2"
OUTDIR="data/${YEAR}-${CRAWL}"

export LC_ALL='C'

./cat.sh $YEAR $CRAWL \
  | sort
  | uniq -c