#!/bin/bash

# TVM SDK Binary Installer
# Automatically detects OS/arch and installs latest TVM binaries

set -e  # Exit on any error

REPO="tvmlabs/tvm-sdk"
TEMP_DIR=$(mktemp -d -t tvm-install-XXXXXX)

log_info() {
    echo -e "\033[0;34m INFO\033[0m" "$@" >&2
}

log_success() {
    echo -e "\033[0;32m   OK\033[0m" "$@" >&2
}

log_warn() {
    echo -e "\033[1;33m WARN\033[0m" "$@" >&2
}

log_error() {
    echo -e "\033[0;31mERROR\033[0m" "$@" >&2
}

cleanup() {
    if [ -d "$TEMP_DIR" ]; then
        rm -rf "$TEMP_DIR"
        log_info "Cleaned up temporary directory"
    fi
}

trap cleanup EXIT

detect_os() {
    case "$(uname -s)" in
        Linux*)
            echo "linux"
            ;;
        Darwin*)
            echo "macos"
            ;;
        *)
            log_error "Unsupported operating system: $(uname -s)"
            exit 1
            ;;
    esac
}

detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64)
            echo "amd64"
            ;;
        aarch64|arm64)
            echo "arm64"
            ;;
        *)
            log_error "Unsupported architecture: $(uname -m)"
            exit 1
            ;;
    esac
}

get_latest_version() {
    log_info "Fetching latest release version..."

    local response
    response=$(curl --silent --fail "https://api.github.com/repos/$REPO/releases/latest") || {
        log_error "Failed to fetch release information from GitHub API"
        exit 1
    }

    if command -v jq &>/dev/null; then
        echo "$response" | jq -r '.tag_name' | sed 's/^v//'
    else
        echo "$response" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/' | sed 's/^v//'
    fi
}

build_download_pattern() {
    local version="$1"
    local os="$2"
    local arch="$3"
    local binary_name="$4"

    if [[ "$version" =~ -[0-9]+$ ]]; then
        version="${version%-*}"
    fi

    if [ "$os" = "linux" ]; then
        echo "${binary_name}-${version}-linux-musl-${arch}.tar.gz"
    else
        echo "${binary_name}-${version}-${os}-${arch}.tar.gz"
    fi
}

download_and_extract() {
    local version="$1"
    local os="$2"
    local arch="$3"
    local binary_name="$4"
    local progress_prefix="$5"

    local pattern
    pattern=$(build_download_pattern "$version" "$os" "$arch" "$binary_name")

    local base_url="https://github.com/$REPO/releases/download/v$version"
    local downloaded=false

    local url="$base_url"/"$pattern"
    local filename="$pattern"

    log_info "$progress_prefix" Attempting to download: "$filename"
    log_info "$progress_prefix" From URL: "$url"

    if curl -sL --fail --retry 3 --retry-delay 2 --retry-connrefused --progress-bar "$url" -o "$TEMP_DIR/$filename" ; then
        downloaded=true
    fi

    if [ "$downloaded" = false ]; then
        log_error "Failed to download $binary_name for $os-$arch"
        return 1
    fi

    log_success "$progress_prefix" "Downloaded $filename"

    log_info "$progress_prefix" "Extracting $filename..."
    (
        cd "$TEMP_DIR" || exit 1

        if ! tar -xzf "$filename" 2>/dev/null; then
            log_error "$progress_prefix" "Failed to extract $filename"
            return 1
        fi

        if [ ! -f "$binary_name" ]; then
            log_error "$progress_prefix" "Could not find $binary_name in extracted archive"
            log_info "$progress_prefix" "Available files in archive:"
            find . -type f | head -10
            return 1
        fi

        cp "$binary_name" "$INSTALL_DIR"/"$binary_name"
    )

    chmod +x "$INSTALL_DIR"/"$binary_name"

    log_success "$progress_prefix" "Installed $binary_name to $INSTALL_DIR"
    return 0
}

check_path() {
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        log_warn ""
        log_warn "$INSTALL_DIR" is not in your PATH. Add this to your shell profile:
        log_warn "export PATH=\"\$INSTALL_DIR:\$PATH\""
        log_warn ""
        log_warn "Or run for bash: echo 'export PATH=\"\$INSTALL_DIR:\$PATH\"' >> ~/.bashrc"
        log_warn "Or run for  zsh: echo 'export PATH=\"\$INSTALL_DIR:\$PATH\"' >> ~/.zshrc"
        log_warn "Or run for fish: echo 'set -x PATH \"\$INSTALL_DIR\" \$PATH' >> ~/.config/fish/config.fish"
        log_warn ""
        log_warn "Note: You may need to restart your shell for the changes to take effect."
        log_warn ""
    fi
}

check_curl() {
    if ! command -v curl &>/dev/null; then
        log_error "curl is not installed. Please install curl and try again."
        exit 1
    fi
}

main() {
    log_info "TVM SDK Binary Installer"
    log_info "========================"

    if [ -z "$INSTALL_DIR" ]; then
        log_error "INSTALL_DIR environment variable not set"
        echo "Usage: "
        echo ""
        echo "   $ INSTALL_DIR=<install_dir> $0"
        echo "   $ TVM_VERSION=<version> INSTALL_DIR=<install_dir> $0"
        echo ""
        exit 1
    fi

    INSTALL_DIR=$(eval echo "$INSTALL_DIR")  # Expand paths like ~
    if [ ! -d "$INSTALL_DIR" ]; then
        log_error "Install directory does not exist: $INSTALL_DIR"
        exit 1
    fi

    check_curl

    local os arch version
    os=$(detect_os)
    arch=$(detect_arch)

    log_info "Detected OS: $os"
    log_info "Detected Architecture: $arch"

    version="${TVM_VERSION:-$(get_latest_version)}"
    if [ -z "$TVM_VERSION" ]; then
        log_info "Using latest version: $version"
        log_info "HINT: You can set the TVM_VERSION environment variable to a specific version (e.g. TVM_VERSION=X.X.X.an)"
    else
        log_info "Using specified version: $version"
    fi

    mkdir -p "$TEMP_DIR"

    local binaries=("tvm-cli" "tvm-debugger")
    local installed_count=0
    local attempted_count=0

    for binary in "${binaries[@]}"; do
        attempted_count=$((attempted_count + 1))
        local progress_prefix="[${attempted_count}/${#binaries[@]}]"
        log_info ""
        log_info "$progress_prefix" Installing "$binary"

        if download_and_extract "$version" "$os" "$arch" "$binary" "$progress_prefix"; then
            installed_count=$((installed_count + 1))
        else
            log_error "$progress_prefix" "Failed to install $binary"
        fi
    done

    log_info ""
    log_info ""
    log_info ""
    log_info "===================="
    log_info "Installation Summary"
    log_info "===================="
    log_success "Successfully installed $installed_count/${#binaries[@]} binaries"

    if [ $installed_count -gt 0 ]; then
        log_info "Binaries installed to: $INSTALL_DIR"

        log_info ""
        log_info "Installed binaries:"
        for binary in "${binaries[@]}"; do
            if [ -f "$INSTALL_DIR/$binary" ]; then
                local version_output
                if [ "$binary" = "tvm-cli" ]; then
                    version_output=$("$INSTALL_DIR"/"$binary" version 2>/dev/null || echo "ERROR: version check failed")
                else
                    version_output=$("$INSTALL_DIR"/"$binary" --version 2>/dev/null || echo "ERROR: version check failed")
                fi
                log_success " * $binary: $version_output"
            fi
        done

        check_path

        log_success "Installation completed successfully!"
    else
        log_error "No binaries were installed successfully"
        exit 1
    fi
}

main "$@"
