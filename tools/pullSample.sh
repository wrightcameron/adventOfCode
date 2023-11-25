#!/bin/bash

# Script for Advent of Code, download the sample input found for each challange.
# Pulls the first sample found between <pre><code>...</pre></code>

# TODO Check if the url doesn't exist, check the head for a 404, would add another request
# TODO Tell the user if the input file is empty, if it is tell the user and maybe not create it.

YEAR=$1
PROBLEM=$2
TMPSOURCEDIR=./source.txt

if [ -z $YEAR -o -z $PROBLEM ]; then
    echo "pullSample.sh <year> <problem>"
    exit 1
fi

url="https://adventofcode.com/$YEAR/day/$PROBLEM"

# Get Advent of Code Challange HTML source
curl -s $url /dev/null > $TMPSOURCEDIR
# Check if curl was successful, exit if it wasn't.
if [ ! $? ]; then
    echo "Error getting html from from $url"
    rm -f $TMPSOURCEDIR
    exit 1
fi

# html=$(curl -s $url 2> /dev/null > source.txt)
cat ./source.txt | sed -n "/<pre>/,/<\/pre>/p" | sed -E "s/<pre><code>|<\/code><\/pre>//g" > ./input.txt

# Clean up
rm -f $TMPSOURCEDIR
