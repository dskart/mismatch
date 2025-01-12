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

get_git_hash() {
    if git rev-parse --git-dir > /dev/null 2>&1; then
        git rev-parse --short HEAD
    else
        log "⚠️" "Not a git repository, using 'latest' as version"
        echo "latest"
    fi
}

while getopts "v:" opt; do
    case $opt in
        v) VERSION="$OPTARG";;
        ?) usage;;
    esac
done

if [ -z "$VERSION" ]; then
    VERSION=$(get_git_hash)
fi


cd ./aws
nvm use

# Deploy using AWS CDK
log "🚀" "Deploying with AWS CDK..."
log "🏷️" "Version: ${VERSION}"
if cdk deploy MismatchStack --parameters ImageTag=${VERSION}
    echo
    log "✅" "Deployment complete"
else 
    echo
    error_log "Deployment failed"
    exit 1
fi