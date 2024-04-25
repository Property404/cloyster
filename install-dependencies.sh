#!/usr/bin/env bash
# Install dependencies needed for Cloyster development
# shellcheck disable=SC2086
set -e

main() {
    local -r USAGE="Usage: $(basename "${0}")"
    local -r HELP="Install CI dependencies

$USAGE

Help:
    ${0}"

    while true; do
        case "$1" in
            -h | --help ) echo "$HELP"; return 0 ;;
            -- ) shift; break ;;
            -* ) echo -e "Unrecognized option: $1\n$USAGE" >&2; return 1 ;;
            * ) break ;;
        esac
    done


    local packages=" gcc git curl "

    # Ubuntu
    if command -v apt-get > /dev/null; then
        sudo apt-get install -y ${packages}
    # macOS
    elif command -v brew > /dev/null; then
        brew install ${packages}
    # fedora
    elif command -v dnf > /dev/null; then
        sudo dnf install -y ${packages}
    # And fuck everyone else
    else
        echo "Unsupported OS" >&2
        return 1
    fi

    # Install RustUp if needed
    if ! command -v rustup > /dev/null; then
        curl --proto '=https' --tlsv1.3 -sSf https://sh.rustup.rs | sh -- -y
        export PATH="${PATH}:${HOME}/.cargo/bin"
    fi

    # Update
    rustup update nightly
}

main "${@}"
