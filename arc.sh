#!/usr/bin/env sh
set -e

find . -type f \
  \( \
  -name "*.rs" \
  -o -name "*.toml" \
  -o -name "*.md" \
  -o -name "*.py" \
  -o -name "*.sh" \
  -o -name "*.yaml" \
  -o -name "*.json" \
  -o -name "*.sol" \
  -o -name "*.c" \
  -o -name "*.cpp" \
  -o -name "*.h" \
  -o -name "*.hpp" \
  \) \
  -not -path "./target/*" \
  -not -path "./.git/*" \
  -not -path "./.idea/*" \
  -not -path "./venv/*" \
  -print0 \
| sort -z \
| LC_ALL=C tar --null -T - -cf - \
| xz -9 -e -T0 > an.tar.xz

