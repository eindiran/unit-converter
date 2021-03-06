#!/usr/bin/env bash
# Installs unit-converter and its manpage
# Supports Linux, macOS, and BSD systems

set -Eeuo pipefail

# TODO: Add some logic here to check that cargo is installed,
#       check that the build succeeds, etc. The script should
#       handle the relevant failures gracefully.
cargo build --release
if [[ "$OSTYPE" == "linux-gnu" ]] ; then
    MAN_PATH=/usr/local/man/man1
    BIN_PATH=/usr/local/sbin
elif [[ "$OSTYPE" == "darwin"* ]] ; then
    MAN_PATH=/usr/local/share/man/man1
    BIN_PATH=/usr/local/sbin
elif [[ "$OSTYPE" == *"bsd" ]] ; then
    MAN_PATH="${MANPREFIX}"/man
    BIN_PATH=/usr/local/sbin
else
    # This might not be a BSD, Linux, or macOS system
    printf "OS is not recognized as a *nix system\n"
    printf "Unable to determine locations to install binaries and manpage\n"
    printf "Try setting system-specific values for MAN_PATH and BIN_PATH;\n"
    printf "Then proceed with the install using these commands:\n\n"
    printf "install -g 0 -o 0 -m 0755 target/release/unit-converter \$BIN_PATH/unit-converter\n"
    printf "install -g 0 -o 0 -m 0644 unit-converter.roff \$MAN_PATH/unit-converter.1\n"
    printf "gzip \$MAN_PATH/unit-converter.1\n\n"
    printf "Exiting the installation process...\n"
    exit 1
fi

install -g 0 -o 0 -m 0755 target/release/unit-converter "${BIN_PATH}"/unit-converter
install -g 0 -o 0 -m 0644 unit-converter.roff "${MAN_PATH}"/unit-converter.1
gzip "${MAN_PATH}"/unit-converter.1
