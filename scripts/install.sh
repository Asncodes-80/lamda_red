#!/bin/sh
set -eu

VERSION=${LMDRED_VERSION:-1.1.0}
BIN_DIR=${LMDRED_INSTALL_DIR:-"$HOME/.local/bin"}
BASE_URL=${LMDRED_RELEASE_BASE_URL:-https://github.com/Asncodes-80/lamda_red/releases/download}
UPDATE_PATH=1

usage() {
    cat <<'EOF'
Install the lmdred compiler from GitHub Releases.

Usage: sh install.sh [--version VERSION] [--bin-dir DIRECTORY] [--no-modify-path]

Defaults: version 1.1.0, directory $HOME/.local/bin.
Set LMDRED_VERSION or LMDRED_INSTALL_DIR to override these defaults.
Run the installer again to upgrade or reinstall.
EOF
}

die() {
    printf 'lmdred: %s\n' "$*" >&2
    exit 1
}

while [ "$#" -gt 0 ]; do
    case "$1" in
        --version|--bin-dir)
            [ "$#" -ge 2 ] || die "Missing value for $1"
            case "$1" in
                --version) VERSION=$2 ;;
                --bin-dir) BIN_DIR=$2 ;;
            esac
            shift 2
            ;;
        --no-modify-path) UPDATE_PATH=0; shift ;;
        -h|--help) usage; exit 0 ;;
        *) die "Unknown argument: $1" ;;
    esac
done
VERSION=${VERSION#lmdred-v}
printf '%s\n' "$VERSION" | grep -Eq '^[0-9]+\.[0-9]+\.[0-9]+$' ||
    die 'Use a numeric release version such as 1.1.0'

case "$(uname -s)" in
    Linux) PLATFORM=unknown-linux-musl ;;
    Darwin) PLATFORM=apple-darwin ;;
    *) die 'This installer supports Linux and macOS. On Windows use install.ps1.' ;;
esac
case "$(uname -m)" in
    x86_64|amd64) ARCH=x86_64 ;;
    aarch64|arm64) ARCH=aarch64 ;;
    *) die 'Supported processor architectures are x64 and ARM64.' ;;
esac
# Prefer the native Apple Silicon binary even when the shell runs in Rosetta.
if [ "$PLATFORM" = apple-darwin ] && [ "$(sysctl -n hw.optional.arm64 2>/dev/null || true)" = 1 ]; then
    ARCH=aarch64
fi
TARGET=$ARCH-$PLATFORM
NAME=lmdred-v$VERSION-$TARGET.tar.gz
URL=${BASE_URL%/}/lmdred-v$VERSION

download() {
    if command -v curl >/dev/null 2>&1; then
        curl --fail --location --retry 3 --silent --show-error --output "$2" "$1"
    elif command -v wget >/dev/null 2>&1; then
        wget -q -O "$2" "$1"
    else
        die 'Install curl or wget to download lmdred.'
    fi
}

command -v tar >/dev/null 2>&1 || die 'Install tar to extract the release.'
TMP_DIR=$(mktemp -d "${TMPDIR:-/tmp}/lmdred.XXXXXXXX")
trap 'rm -rf "$TMP_DIR"' EXIT
trap 'exit 1' HUP INT TERM
printf 'Downloading lmdred %s for %s...\n' "$VERSION" "$TARGET"
download "$URL/$NAME" "$TMP_DIR/$NAME" ||
    die "Could not download $URL/$NAME. Check that this compiler release is published."
download "$URL/$NAME.sha256" "$TMP_DIR/$NAME.sha256" ||
    die 'Could not download the release checksum.'
EXPECTED=$(awk 'NR == 1 { print $1 }' "$TMP_DIR/$NAME.sha256")
case "$EXPECTED" in ''|*[!a-fA-F0-9]*) die 'Invalid release checksum.' ;; esac
[ "${#EXPECTED}" -eq 64 ] || die 'Invalid release checksum length.'
if command -v sha256sum >/dev/null 2>&1; then
    ACTUAL=$(sha256sum "$TMP_DIR/$NAME" | awk '{ print $1 }')
elif command -v shasum >/dev/null 2>&1; then
    ACTUAL=$(shasum -a 256 "$TMP_DIR/$NAME" | awk '{ print $1 }')
else
    die 'Install sha256sum or shasum to verify the download.'
fi
[ "$ACTUAL" = "$EXPECTED" ] || die 'SHA-256 verification failed.'
# Extract only the executable, keeping the archive's other paths out of the install.
tar -xzf "$TMP_DIR/$NAME" -C "$TMP_DIR" lmdred
[ -f "$TMP_DIR/lmdred" ] && [ ! -L "$TMP_DIR/lmdred" ] || die 'The archive has no regular lmdred binary.'
chmod 755 "$TMP_DIR/lmdred"
"$TMP_DIR/lmdred" --version

mkdir -p "$BIN_DIR"
BIN_DIR=$(cd "$BIN_DIR" && pwd -P)
STAGED=$(mktemp "$BIN_DIR/.lmdred.XXXXXXXX")
trap 'rm -rf "$TMP_DIR"; rm -f "$STAGED"' EXIT
cp "$TMP_DIR/lmdred" "$STAGED"
chmod 755 "$STAGED"
mv -f "$STAGED" "$BIN_DIR/lmdred"

quote_shell() {
    printf "'"
    printf '%s' "$1" | sed "s/'/'\\\\''/g"
    printf "'"
}

if [ "$UPDATE_PATH" -eq 1 ]; then
    QUOTED=$(quote_shell "$BIN_DIR")
    PATH_LINE="export PATH=$QUOTED:\"\$PATH\""
    add_profile() {
        mkdir -p "$(dirname "$1")"
        if [ ! -f "$1" ] || ! grep -Fqx "$PATH_LINE" "$1"; then
            printf '\n# Lamda RED compiler\n%s\n' "$PATH_LINE" >> "$1"
        fi
    }
    add_profile "$HOME/.profile"
    case "${SHELL:-/bin/sh}" in
        */bash)
            add_profile "$HOME/.bashrc"
            if [ -f "$HOME/.bash_profile" ]; then
                add_profile "$HOME/.bash_profile"
            elif [ -f "$HOME/.bash_login" ]; then
                add_profile "$HOME/.bash_login"
            fi
            ;;
        */zsh)
            add_profile "${ZDOTDIR:-$HOME}/.zshrc"
            add_profile "${ZDOTDIR:-$HOME}/.zprofile"
            ;;
        */fish)
            FISH_FILE=${XDG_CONFIG_HOME:-$HOME/.config}/fish/conf.d/lmdred.fish
            mkdir -p "$(dirname "$FISH_FILE")"
            printf 'if not contains -- %s $PATH\n    set -gx PATH %s $PATH\nend\n' "$QUOTED" "$QUOTED" > "$FISH_FILE"
            ;;
    esac
    printf 'Installed %s/lmdred. Open a new terminal, then run:\n' "$BIN_DIR"
else
    printf 'Installed %s/lmdred. Add this directory to PATH, then run:\n' "$BIN_DIR"
fi
printf '  lmdred sample.zz my-new-red\n'
