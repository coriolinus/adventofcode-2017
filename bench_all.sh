#!/bin/bash
TIMEFORMAT=%R
for dir in *; do
   if [ -d $dir -a -f "$dir/Cargo.toml" -a -f "$dir/src/lib.rs" ]; then
      (
         cd $dir
         echo "Preparing $dir..."
         cargo update --quiet
         cargo build --release --quiet
         if [ $? != 0 ]; then
            echo "Build failed for $dir"
         else
            if [ -x "target/release/$dir" ]; then
               time ( target/release/$dir >/dev/null )
            else
               echo "...executable not found"
            fi
            echo
         fi
      )
   fi
done
