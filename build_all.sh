#!/bin/bash

rv="0"
for dir in *; do
   if [ -d $dir -a -f "$dir/Cargo.toml" -a -f "$dir/src/lib.rs" ]; then
      (
         cd $dir
         cargo build --quiet
         if [ $? != 0 ]; then
            echo "Build failed for $dir"
            rv=$?
         fi
      )
   fi
done

exit $rv
