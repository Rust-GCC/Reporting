#!/bin/sh
DATE=$(date -I)
AUTHOR="Philip Herron, Pierre-Emmanuel Patry, Arthur Cohen"

if [ ! -z $1 ]; then
  PATH_TO_SRC="-l $1"
fi

if [ ! -z $2 ]; then
  TOKEN="--token $2"
fi

if [ -f "Cargo.toml" ]; then

  CMD="cargo run -- "
else
  CMD="./generator"
fi

$CMD -k weekly -d "$DATE" -a "$AUTHOR" $PATH_TO_SRC $TOKEN
