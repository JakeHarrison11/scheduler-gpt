#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SRC="$ROOT_DIR/scheduler-gpt.rs"
BIN="$ROOT_DIR/scheduler-gpt"
TEST_DIR="${1:-$ROOT_DIR/pa1-testfiles-1}"

if [[ ! -d "$TEST_DIR" ]]; then
	echo "Test directory not found: $TEST_DIR"
	exit 1
fi

if [[ ! -f "$SRC" ]]; then
	echo "Source file not found: $SRC"
	exit 1
fi

if [[ ! -x "$BIN" || "$SRC" -nt "$BIN" ]]; then
	echo "Compiling scheduler-gpt.rs..."
	rustc "$SRC" -o "$BIN"
fi

shopt -s nullglob
inputs=("$TEST_DIR"/*.in)

if [[ ${#inputs[@]} -eq 0 ]]; then
	echo "No .in files found in $TEST_DIR"
	exit 1
fi

pass=0
fail=0

for input in "${inputs[@]}"; do
	base="${input%.in}"
	expected="$base.out"
	name="$(basename "$base")"

	if [[ ! -f "$expected" ]]; then
		echo "[SKIP] $name (missing expected output: $(basename "$expected"))"
		continue
	fi

	tmpdir="$(mktemp -d)"
	trap 'rm -rf "$tmpdir"' EXIT

	input_copy="$tmpdir/$(basename "$input")"
	cp "$input" "$input_copy"

	"$BIN" "$input_copy" >/dev/null 2>&1

	actual="$tmpdir/$(basename "$expected")"
	if diff -u "$expected" "$actual" >/dev/null; then
		echo "[PASS] $name"
		pass=$((pass + 1))
	else
		echo "[FAIL] $name"
		echo "  Expected: $expected"
		echo "  Actual:   $actual"
		diff -u "$expected" "$actual" || true
		fail=$((fail + 1))
	fi

	rm -rf "$tmpdir"
	trap - EXIT
done

echo
echo "Passed: $pass"
echo "Failed: $fail"

if [[ $fail -ne 0 ]]; then
	exit 1
fi
