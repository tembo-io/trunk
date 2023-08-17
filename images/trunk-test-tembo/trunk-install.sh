#!/bin/bash
failed_extensions=()
file='/tmp/extensions.txt'
extension_count=$(<$file wc -l)
lines=$(cat $file)
for line in $lines
do
        trunk install $line
        if [ $? -ne 0 ]; then
            echo "trunk install command failed"
            failed_extensions+=("$line")
        fi
        printf "\n\n"
done
IFS=$'\n' extensions=(`psql postgres://postgres:postgres@localhost:5432 -tA postgres -c 'select name from pg_available_extensions;'`)
for ext in "${extensions[@]}"
do
        psql postgres://postgres:postgres@localhost:5432 -c "create extension if not exists \"$ext\" cascade;"
        if [ $? -ne 0 ]; then
            echo "CREATE EXTENSION command failed"
            failed_extensions+=("$ext")
        fi
        printf "\n\n"
done
failure_count=${#failed_extensions[@]}
success=$(($extension_count-$failure_count))
success_percent=$(awk "BEGIN { pc=100*${success}/${extension_count}; i=int(pc); print (pc-i<0.5)?i:i+1 }")
failure_percent=$(awk "BEGIN { pc=100*${failure_count}/${extension_count}; i=int(pc); print (pc-i<0.5)?i:i+1 }")
printf "***SUCCESS RATE***\n"
echo "$success / $extension_count ($success_percent%)"
printf "\n\n***FAILURE RATE***\n"
echo "$failure_count / $extension_count ($failure_percent%)"
printf "\n\n***FAILED EXTENSIONS***\n"
for failed in "${failed_extensions[@]}"
do
      echo $failed
done
