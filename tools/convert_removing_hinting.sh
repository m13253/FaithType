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

mkdir -p ./out

for i in "$@"
do
    INPUT_FILE="$i"
    OUTPUT_FILE="./out/$(basename "$i")"
    echo "$ cargo run --quiet -- -o \"$OUTPUT_FILE\" -- \"$INPUT_FILE\""
    cargo run --quiet -- -o "$OUTPUT_FILE" -- "$INPUT_FILE"
    echo "Converted $INPUT_FILE -> $OUTPUT_FILE"
    echo
done
