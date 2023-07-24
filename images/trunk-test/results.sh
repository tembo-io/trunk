#!/bin/bash
failures=()
file='/tmp/output.txt'
lines=$(cat $file)
while read line;
do
      printf "\n\n"
      if [[ $line == *"ERROR"* ]]; then
              echo "FOUND AN ERROR"
              failures+=("$line")
      fi
      printf "\n\n"
done < $file
wc -l $file
cat $file | grep -i error
#echo "FAILURES: "
echo ${#failures[@]}
printf "\n"
for failure in "${failures[@]}"
do
      echo $failure
done