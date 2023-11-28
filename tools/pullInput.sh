#!/bin/bash

YEAR=$1
PROBLEM=$2

if [ -z $YEAR -o -z $PROBLEM ]; then
    echo "pullSample.sh <year> <problem>"
    exit 1
fi

url="https://adventofcode.com/$YEAR/day/$PROBLEM/input"

wget url