#!/bin/sh

# A script to run e2e tests

BIN_PATH="./target/release"

for case in e2e/*; do
  printf "TEST: %s" "$case"
  if [ "$($BIN_PATH/hashdir "$case/a")" = "$($BIN_PATH/hashdir "$case/b")" ]; then
    printf " OK\n"
  else
    printf " FAIL\n"
    exit 1
  fi
done
