#!/bin/sh
DATE=$(date -I)
AUTHOR="Philip Herron, Pierre-Emmanuel Patry, Arthur Cohen"

if [ ! -z $1 ]; then
  PATH_TO_SRC="-l $1"
fi


if [ -f "Cargo.toml" ]; then

  CMD="cargo run -- "
else
  CMD="./generator"
fi

$CMD -k monthly -d "$DATE" -a "$AUTHOR" $PATH_TO_SRC --token "$GH_TOKEN"
