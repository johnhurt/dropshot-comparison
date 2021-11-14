#!/bin/sh

killall dropshot-main 2> /dev/null
killall dropshot-pr 2> /dev/null

set -e

### PR Branch

echo Building PR Branch Server

cargo build --manifest-path dropshot-pr/Cargo.toml --release > /dev/null 2> /dev/null &

echo Starting PR Branch Server

cargo flamegraph --bin dropshot-pr --root -o pr.svg /dev/null 2> /dev/null &

PROCESS_ID=$!

sleep 5

echo Starting PR Branch Test

drill --benchmark ./benchmark.yml -s -q -n

kill $PROCESS_ID
sudo killall dtrace

### Release branch

echo Building Release Branch Server

cargo build --manifest-path dropshot-main/Cargo.toml --release > /dev/null 2> /dev/null &

echo Starting Release Branch Server

cargo flamegraph --bin dropshot-main -o main.svg --root > /dev/null 2> /dev/null &

PROCESS_ID=$!

sleep 5

echo Starting Release Branch Test

drill --benchmark ./benchmark.yml -s -q -n

kill $PROCESS_ID
sudo killall dtrace

sleep 3
