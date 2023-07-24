#!/bin/bash
failures=0
file='/tmp/extensions.txt'
lines=$(cat $file)
for line in $lines
do
        trunk install $line
        psql -c "create extension \"$line\" cascade;"
        if [ $? -eq 0 ]; then
            echo "Command succeeded"
        else
            echo "Command failed"
            let "failures++"
        fi
        printf "\n\n"
done
echo "FAILURES: $failures"
