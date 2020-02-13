#!/usr/bin/env bash

YEAR="$1"
CRAWL="$2"
OUTDIR="data/${YEAR}-${CRAWL}"

export LC_ALL='C'

ls $OUTDIR \
  | xargs -I% cat $OUTDIR/% \
  | sed 's|song.*$|Song of Solomon|ig;s|Revelations|Revelation|ig;s|Psalm |Psalms |ig;s|^1|First|ig;s|^2|Second|ig;s|^3|Third|ig' \
  | tr '[:lower:]' '[:upper:]' \
  | sort \
  | uniq -c > "$OUTDIR.txt"