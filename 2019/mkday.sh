#!/bin/sh
set -eux

D=$(printf '%02d' ${1})
for X in _a.nim _b.nim _a.md _b.md .data; do
  touch "day${D}${X}"
done

chmod +x "day${D}_a.nim"
chmod +x "day${D}_b.nim"