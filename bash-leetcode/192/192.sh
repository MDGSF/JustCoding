# Read from the file words.txt and output the word frequency list to stdout.
cat words.txt | xargs -n 1 | sort | uniq -c | sort -nr | awk '{print $2" "$1}'
