#!/bin/bash

grep -o "branch=master\?#[a-f0-9]*" Cargo.lock > iced_hash.txt
cat iced_hash.txt
sed '1!d' iced_hash.txt > tmp0.txt
sed 's/branch=master?#//g' > tmp1.txt
cat tmp1.txt