#!/bin/sh
cargo strip $2
scp $2 $1:~/
ssh -t $1 'sudo ./cyclic'
