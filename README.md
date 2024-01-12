# volume-control
# A simple volume daemon that allows volume keys to work.

Add these commands to you bash file:

Change key binding for raising volume
./target/release/volume-control changekey up KEY

Change key binding for lowering volume
./target/release/volume-control changekey down KEY

Change key binding for muting volume
./target/release/volume-control changekey mute KEY
