#/bin/sh
list=`ls src/bin|sort -g|awk -F . '{print $1}'|grep -v 12`
for each in $list; do 
  echo Testing Day $each
  cargo run -q --release --bin $each|grep Time
  sleep 1
done
