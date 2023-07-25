#!/bin/bash
failure_count=0
failed_extensions=()
file='/tmp/extensions.txt'
extension_count=$(<$file wc -l)
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
printf "*** FAILURE COUNT ***: \n$failure_count / $extension_count"
printf "\n\n*** FAILED EXTENSIONS ***:\n"
for failed in "${failed_extensions[@]}"
do
      echo $failed
done
