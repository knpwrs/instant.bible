#!/usr/bin/env bash

BASE_URL="https://commoncrawl.s3.amazonaws.com"
YEAR="$1"
CRAWL="$2"
PROC=${3:-2}
OUTDIR="data/${YEAR}-${CRAWL}"

cat books.multi.txt | sed 's|$| \\d{1,2}:\\d{1,2}|g' > verse.regex.build
echo >> verse.regex.build
cat books.single.txt | sed 's|$| (?:1:)?\\d{1,2}|g' >> verse.regex.build
echo >> verse.regex.build
cat books.big.txt | sed 's|$| \\d{1,3}:\\d{1,3}|g' >> verse.regex.build
echo >> verse.regex.build

mkdir -p $OUTDIR

curl -s "$BASE_URL/crawl-data/CC-MAIN-$YEAR-$CRAWL/wet.paths.gz" \
  | zcat \
  | xargs -n1 -P $PROC -I% ./_crawl.sh $BASE_URL % $OUTDIR