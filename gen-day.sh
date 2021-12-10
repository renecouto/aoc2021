#!/bin/bash
set -ex
mkdir day-$1
cp -r template/src day-$1/
cp template/Cargo.toml day-$1/
sed -i "s/template/day-$1/g" day-$1/Cargo.toml
touch day-$1/input.txt
