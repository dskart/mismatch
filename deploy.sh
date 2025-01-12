#!/bin/bash

# Exit on any error
set -e

# Function to display usage
usage() {
    echo "📋 Usage: $0 [-v VERSION]"
    echo "  -v: Additional version tag (default: git commit hash)"
    exit 1
}

log() {
    local emoji="$1"
    local message="$2"
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $emoji $message"
}

error_log() {
    log "❌" "$1"
}

get_version_identifier() {
    # Check if current commit has a tag
    local git_tag=$(git describe --exact-match --tags HEAD 2>/dev/null)
    if [ -n "$git_tag" ]; then
        echo "$git_tag"
    else
        # Fallback to commit hash
        git rev-parse --short HEAD
    fi
}

while getopts "v:" opt; do
    case $opt in
        v) VERSION="$OPTARG";;
        ?) usage;;
    esac
done

if [ -z "$VERSION" ]; then
    VERSION=$(get_version_identifier)
fi

cd ./aws

# Source NVM if available
if [ -f "$HOME/.nvm/nvm.sh" ]; then
    . "$HOME/.nvm/nvm.sh"
elif [ -f "/usr/local/opt/nvm/nvm.sh" ]; then
    . "/usr/local/opt/nvm/nvm.sh"
else
    error_log "NVM not found. Please install NVM first."
    exit 1
fi

# Try to use project's Node.js version or fallback to default
if [ -f ".nvmrc" ]; then
    nvm use || nvm use default
else
    log "ℹ️" "No .nvmrc found, using default Node version"
    nvm use default
fi

# Deploy using AWS CDK
log "🚀" "Deploying with AWS CDK..."
log "🏷️" "Version: ${VERSION}"
if cdk deploy MismatchStack --parameters ImageTag=${VERSION}; then
    echo
    log "✅" "Deployment complete"
else 
    echo
    error_log "Deployment failed"
    exit 1
fi