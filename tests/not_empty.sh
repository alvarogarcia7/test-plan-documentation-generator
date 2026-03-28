#!/usr/bin/env bash

set -euo pipefail

file="$1"
lines=$(wc -l "${file}"| awk '{print $1}')

if [[ $lines -gt 0 ]]; then
  echo "[DEBUG] the file '${file}' contains '$lines' lines"
  exit_code=0
else
  echo "[ERROR] the file '${file}' contains '$lines' lines"
  exit_code=1
fi

exit $exit_code

