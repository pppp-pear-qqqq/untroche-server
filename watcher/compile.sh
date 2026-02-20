#!/bin/sh
filepath=$1

# ts でないファイルが渡された場合は無視
if ! echo "$filepath" | grep -q '\.ts$'; then
    exit 0
fi

if [ ! -s "$filepath" ]; then
    # echo "[TS Watcher] File is empty, waiting for save: $filepath"
    exit 0
fi

outfile=$(echo "$filepath" | sed 's|/resource/ts/|/resource/script/|' | sed 's/\.ts$/.js/')
mkdir -p "$(dirname "$outfile")"

# echo "[TS Watcher] Compiling: $filepath"
esbuild "$filepath" --outfile="$outfile" --tsconfig=/app/tsconfig.json --log-level=warning
