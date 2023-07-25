#!/bin/bash
failure_count=0
failed_extensions=()
file='/tmp/extensions.txt'
lines=$(cat $file)
for line in $lines
do
        trunk install $line
        psql -c "create extension \"$line\" cascade;"
        if [ $? -ne 0 ]; then
            echo "CREATE EXTENSION command failed"
            let "failure_count++"
            failed_extensions+=("$line")
        fi
        printf "\n\n"
done
echo "*** FAILURE COUNT ***: $failure_count"
printf "\n*** FAILED EXTENSIONS ***:\n"
for failed in "${failed_extensions[@]}"
do
      echo $failed
done
