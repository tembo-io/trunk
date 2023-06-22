#!/bin/bash
file='/tmp/extensions.txt'
lines=$(cat $file)
for line in $lines
do
        trunk install $line
        psql -c "create extension \"$line\" cascade;"
        printf "\n\n"
done
