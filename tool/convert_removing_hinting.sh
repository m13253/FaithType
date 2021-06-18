#!/bin/bash

set -e

if [ "$#" -eq 0 ]
then
    echo
    echo "Usage $0 input_file_0.ttf input_file_2.otf input_file_3.ttc ..."
    echo
    echo "Output files will be placed in ./out"
    echo
    exit 0
fi

mkdir -p ./tmp
mkdir -p ./out

for i in "$@"
do
    INPUT_FILE="$i"
    TEMP_FILE="./tmp/$(basename "$i")"
    OUTPUT_FILE="./out/$(basename "$i")"
    echo "$ ttfautohint --dehint -- \"$INPUT_FILE\" \"$TEMP_FILE\""
    ttfautohint --dehint -- "$INPUT_FILE" "$TEMP_FILE"
    echo "$ cargo run --quiet --release -- -o \"$OUTPUT_FILE\" --remove-hinting -- \"$INPUT_FILE\""
    cargo run --quiet --release -- -o "$OUTPUT_FILE" --remove-bitmap --remove-hinting --modify-gasp -- "$INPUT_FILE"
    rm "$TEMP_FILE"
    echo "Converted $INPUT_FILE -> $OUTPUT_FILE"
    echo
done

rmdir --ignore-fail-on-non-empty ./tmp
