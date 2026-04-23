#!/usr/bin/env bash

set -u

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

REPORT_ROOT="${TVM_PRIORITY_COVERAGE_REPORT_ROOT:-$ROOT_DIR/target/priority-feature-coverage}"
TIMESTAMP="${TVM_PRIORITY_COVERAGE_TIMESTAMP:-$(date '+%Y%m%d-%H%M%S')}"
RUN_ID="priority-feature-coverage-$TIMESTAMP"
RUN_DIR="$REPORT_ROOT/$RUN_ID"
if [[ -e "$RUN_DIR" ]]; then
  RUN_ID="$RUN_ID-$$"
  RUN_DIR="$REPORT_ROOT/$RUN_ID"
fi

LOG_DIR="$RUN_DIR/logs"
HTML_DIR="$RUN_DIR/html"
SUMMARY_TSV="$RUN_DIR/steps.tsv"
MANIFEST="$RUN_DIR/manifest.md"
TEXT_REPORT="$RUN_DIR/coverage-summary.txt"
LCOV_REPORT="$RUN_DIR/lcov.info"
ARCHIVE="$REPORT_ROOT/$RUN_ID.tar.gz"

STARTED_AT="$(date '+%Y-%m-%d %H:%M:%S %Z')"
GIT_REV="$(git -C "$ROOT_DIR" rev-parse --short HEAD 2>/dev/null || true)"
GIT_STATUS="$(git -C "$ROOT_DIR" status --short 2>/dev/null || true)"

export CARGO_TERM_COLOR="${CARGO_TERM_COLOR:-never}"

detect_cargo_target_dir() {
  local metadata
  local target_dir
  local link_path
  local workspace_target

  if [[ -n "${TVM_PRIORITY_COVERAGE_CARGO_TARGET_DIR:-}" ]]; then
    printf "%s" "$TVM_PRIORITY_COVERAGE_CARGO_TARGET_DIR"
    return 0
  fi

  metadata="$(cd "$ROOT_DIR" && cargo metadata --format-version 1 --no-deps 2>/dev/null)" || true
  if [[ -n "$metadata" ]]; then
    if command -v jq >/dev/null 2>&1; then
      target_dir="$(printf "%s" "$metadata" | jq -r '.target_directory // empty')"
    else
      target_dir="$(printf "%s" "$metadata" | sed -n 's/.*"target_directory":"\([^"]*\)".*/\1/p')"
    fi
    if [[ -n "$target_dir" && "$target_dir" != "null" ]]; then
      workspace_target="$ROOT_DIR/target"
      if [[ -L "$workspace_target" ]]; then
        if [[ "$(cd "$workspace_target" 2>/dev/null && pwd -P)" == "$(cd "$target_dir" 2>/dev/null && pwd -P)" ]]; then
          printf "%s" "$workspace_target"
          return 0
        fi
      elif [[ ! -e "$workspace_target" ]]; then
        ln -s "$target_dir" "$workspace_target"
        printf "%s" "$workspace_target"
        return 0
      fi
      case "$target_dir" in
        "$ROOT_DIR"/*)
          printf "%s" "$target_dir"
          ;;
        *)
          link_path="$ROOT_DIR/target/cargo-metadata-target"
          mkdir -p "$ROOT_DIR/target"
          if [[ -L "$link_path" ]]; then
            if [[ "$(readlink "$link_path")" != "$target_dir" ]]; then
              ln -sfn "$target_dir" "$link_path"
            fi
          elif [[ ! -e "$link_path" ]]; then
            ln -s "$target_dir" "$link_path"
          else
            printf "coverage target link exists and is not a symlink: %s\n" "$link_path" >&2
            return 1
          fi
          printf "%s" "$link_path"
          ;;
      esac
      return 0
    fi
  fi

  printf "%s" "$ROOT_DIR/target"
}

export CARGO_TARGET_DIR="$(detect_cargo_target_dir)"
mkdir -p "$LOG_DIR" "$HTML_DIR" || exit 1

IGNORE_REGEX="${TVM_PRIORITY_COVERAGE_IGNORE_REGEX:-(/tests?/|/benches/)}"

if [[ -n "${TVM_PRIORITY_COVERAGE_JOBS:-}" ]]; then
  CARGO_JOBS=(--jobs "$TVM_PRIORITY_COVERAGE_JOBS")
else
  CARGO_JOBS=()
fi

REPORT_PACKAGES=(
  -p tvm_abi
  -p tvm_block
  -p tvm_block_json
  -p tvm_client
  -p tvm_executor
  -p tvm_sdk
  -p tvm_types
  -p tvm_vm
)

quote_cmd() {
  local out=""
  local arg
  local quoted

  for arg in "$@"; do
    printf -v quoted "%q" "$arg"
    out="$out$quoted "
  done

  printf "%s" "${out% }"
}

printf "id\ttitle\tstatus\texit_code\tduration_seconds\tlog\n" > "$SUMMARY_TSV"

total=0
passed=0
failed=0

run_step() {
  local id="$1"
  local title="$2"
  shift 2

  local log_file="$LOG_DIR/$id.log"
  local started_epoch
  local finished_epoch
  local duration
  local exit_code
  local status_label
  local cmd_display

  total=$((total + 1))
  cmd_display="$(quote_cmd "$@")"

  echo "[$id] $title"
  echo "  $cmd_display"

  started_epoch="$(date '+%s')"
  (
    cd "$ROOT_DIR" || exit 1
    "$@"
  ) > "$log_file" 2>&1
  exit_code=$?
  finished_epoch="$(date '+%s')"
  duration=$((finished_epoch - started_epoch))

  if [[ "$exit_code" -eq 0 ]]; then
    status_label="PASS"
    passed=$((passed + 1))
  else
    status_label="FAIL"
    failed=$((failed + 1))
  fi

  printf "%s\t%s\t%s\t%s\t%s\t%s\n" \
    "$id" "$title" "$status_label" "$exit_code" "$duration" "logs/$id.log" >> "$SUMMARY_TSV"

  {
    printf "## %s\n\n" "$id"
    printf -- "- title: %s\n" "$title"
    printf -- "- status: %s\n" "$status_label"
    printf -- "- exit_code: %s\n" "$exit_code"
    printf -- "- duration_seconds: %s\n" "$duration"
    printf -- "- command: \`%s\`\n\n" "$cmd_display"
  } >> "$MANIFEST"

  echo "  $status_label in ${duration}s"
}

cat > "$MANIFEST" <<EOF
# TVM priority feature coverage

- run_id: \`$RUN_ID\`
- started_at: $STARTED_AT
- git_rev: \`$GIT_REV\`
- cargo_target_dir: \`$CARGO_TARGET_DIR\`
- ignored_sources_regex: \`$IGNORE_REGEX\`

## Priority downstream feature matrix

- \`tvm_abi\`, \`tvm_block\`, \`tvm_block_json\`, \`tvm_sdk\`, \`tvm_types\`: default features.
- \`tvm_vm\`: \`gosh\` feature. This repo default also enables \`gosh\`.
- \`tvm_executor\`: default features plus \`signature_with_id\`.
- \`tvm_client\`: \`default-features = false\`, features \`std\` and \`rustls-tls-webpki-roots\`.
- \`tests/rust/aerospike_store\` dependency subset is covered by \`tvm_block\` and \`tvm_types\` default feature runs.

## Coverage runs

EOF

echo "Coverage run directory: $RUN_DIR"

run_step \
  "clean" \
  "clean old cargo-llvm-cov artifacts for this workspace" \
  cargo llvm-cov clean --workspace --color never

run_step \
  "tvm-abi-default-lib-release" \
  "coverage: cargo test --release --lib for tvm_abi default features" \
  cargo llvm-cov --no-report --release --lib --no-fail-fast "${CARGO_JOBS[@]}" -p tvm_abi

run_step \
  "tvm-block-types-default-lib-release" \
  "coverage: cargo test --release --lib for tvm_block and tvm_types default features" \
  cargo llvm-cov --no-report --release --lib --no-fail-fast "${CARGO_JOBS[@]}" -p tvm_block -p tvm_types

run_step \
  "tvm-block-json-default-lib-release" \
  "coverage: cargo test --release --lib for tvm_block_json default features" \
  cargo llvm-cov --no-report --release --lib --no-fail-fast "${CARGO_JOBS[@]}" -p tvm_block_json

run_step \
  "tvm-sdk-default-lib-release" \
  "coverage: cargo test --release --lib for tvm_sdk default features" \
  cargo llvm-cov --no-report --release --lib --no-fail-fast "${CARGO_JOBS[@]}" -p tvm_sdk

run_step \
  "tvm-vm-gosh-lib-release" \
  "coverage: cargo test --release --lib for tvm_vm with gosh" \
  cargo llvm-cov --no-report --release --lib --no-fail-fast "${CARGO_JOBS[@]}" -p tvm_vm --features tvm_vm/gosh

run_step \
  "tvm-executor-signature-with-id-vm-gosh-lib-release" \
  "coverage: cargo test --release --lib for tvm_executor with signature_with_id and unified tvm_vm/gosh" \
  cargo llvm-cov --no-report --release --lib --no-fail-fast "${CARGO_JOBS[@]}" -p tvm_executor --features tvm_executor/signature_with_id,tvm_vm/gosh

run_step \
  "tvm-client-std-rustls-webpki-roots-lib-release" \
  "coverage: cargo test --release --lib for tvm_client without defaults, with std and rustls webpki roots" \
  cargo llvm-cov --no-report --release --lib --no-fail-fast "${CARGO_JOBS[@]}" -p tvm_client --no-default-features --features tvm_client/std,tvm_client/rustls-tls-webpki-roots

run_step \
  "coverage-html" \
  "generate merged HTML coverage report" \
  cargo llvm-cov report "${REPORT_PACKAGES[@]}" --release --html --output-dir "$RUN_DIR" --ignore-filename-regex "$IGNORE_REGEX" --color never

run_step \
  "coverage-text" \
  "generate merged text coverage summary" \
  cargo llvm-cov report "${REPORT_PACKAGES[@]}" --release --text --output-path "$TEXT_REPORT" --ignore-filename-regex "$IGNORE_REGEX" --color never

run_step \
  "coverage-lcov" \
  "generate merged lcov coverage data" \
  cargo llvm-cov report "${REPORT_PACKAGES[@]}" --release --lcov --output-path "$LCOV_REPORT" --ignore-filename-regex "$IGNORE_REGEX" --color never

FINISHED_AT="$(date '+%Y-%m-%d %H:%M:%S %Z')"

{
  printf "\n## Result\n\n"
  printf -- "- finished_at: %s\n" "$FINISHED_AT"
  printf -- "- steps_total: %s\n" "$total"
  printf -- "- steps_passed: %s\n" "$passed"
  printf -- "- steps_failed: %s\n" "$failed"
  printf -- "- html_index: \`html/index.html\`\n"
  printf -- "- text_summary: \`coverage-summary.txt\`\n"
  printf -- "- lcov: \`lcov.info\`\n"
  if [[ -n "$GIT_STATUS" ]]; then
    printf "\n## Dirty worktree at run time\n\n"
    printf "\`\`\`text\n%s\n\`\`\`\n" "$GIT_STATUS"
  fi
} >> "$MANIFEST"

tar -czf "$ARCHIVE" -C "$REPORT_ROOT" "$RUN_ID"

echo "HTML coverage: $HTML_DIR/index.html"
echo "Text summary: $TEXT_REPORT"
echo "LCOV: $LCOV_REPORT"
echo "Archive: $ARCHIVE"

if [[ "$failed" -gt 0 ]]; then
  exit 1
fi
