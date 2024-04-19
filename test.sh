#!/usr/bin/env bash
# shellcheck disable=SC2086
set -e

declare -r CFLAGS="-Wall -Wextra -std=c2x -pedantic -Werror"

main() {
    local -r temp_dir="$(mktemp -d)"
    local -r aout="${temp_dir}/a.out"

    cargo build

    for test_case in c_test_cases/*.c; do
        echo -n "Testing ${test_case}...";

        gcc ${CFLAGS} "${test_case}" -o "${aout}"
        GLIBC_OUTPUT="$(${aout})"

        gcc ${CFLAGS} -ffreestanding -nostdlib "${test_case}" \
            target/debug/libcloyster.a -o "${aout}"
        CLOYSTER_OUTPUT="$(${aout})"

        if [[ "${GLIBC_OUTPUT}" != "${CLOYSTER_OUTPUT}" ]]; then
            echo "Differing output" >&2
            echo "glibc: ${GLIBC_OUTPUT}" >&2
            echo "cloyster: ${CLOYSTER_OUTPUT}" >&2
            return 1
        fi
        echo "OK"
    done

    rm "${aout}"
    rmdir "${temp_dir}"
}

main