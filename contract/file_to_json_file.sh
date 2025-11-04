#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   ./file_to_json_file.sh [files...]
# If no files are provided, defaults to processing forged.txt and non_forged.txt
cd ..
if [ "$#" -gt 0 ]; then
  files=("$@")
else
  files=("contract/forged.txt" \
         "contract/non_forged.txt"\
	 "contract/ord.txt")
fi

for f in "${files[@]}"; do
  if [ ! -f "$f" ]; then
    echo "Skipping missing file: $f" >&2
    continue
  fi

  out="${f%.*}.json"

  awk '
    BEGIN { printf("{\"cell\":\"") }
    {
      printf "%s", $0
    }
    END { printf("\"}\n") }
  ' "$f" > "$out"

  echo "Wrote $out"
done


