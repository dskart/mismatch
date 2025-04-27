#!/bin/bash

# Exit on any error
set -e

# Default values
DOCKERFILE="Dockerfile"
BUILD_CONTEXT="."
REGISTRY="192.168.1.241:5000"
REPOSITORY="mismatch"

# Function to display usage
usage() {
    echo "📋 Usage: $0 [-r REGISTRY] [-v VERSION] [-f DOCKERFILE] [-c BUILD_CONTEXT]"
    echo "  -r: Registry URI (default: ${REGISTRY})"
    echo "  -v: Additional version tag (default: git commit hash)"
    echo "  -f: Dockerfile path (default: Dockerfile)"
    echo "  -c: Build context path (default: current directory)"
    exit 1
}

# Function to log messages with emoji
log() {
    local emoji="$1"
    local message="$2"
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $emoji $message"
}

# Function to log error messages
error_log() {
    log "❌" "$1"
}

# Function to get git commit hash
get_git_hash() {
    if git rev-parse --git-dir > /dev/null 2>&1; then
        git rev-parse --short HEAD
    else
        log "⚠️" "Not a git repository, using 'latest' as version"
        echo "latest"
    fi
}

get_git_tag() {
    # Check if current commit has a tag
    local git_tag=$(git describe --exact-match --tags HEAD 2>/dev/null)
    if [ -n "$git_tag" ]; then
        echo "$git_tag"
    else
        echo ""
    fi
}

# Parse command line arguments
while getopts "r:v:f:c:" opt; do
    case $opt in
        r) REGISTRY="$OPTARG";;
        v) VERSION="$OPTARG";;
        f) DOCKERFILE="$OPTARG";;
        c) BUILD_CONTEXT="$OPTARG";;
        ?) usage;;
    esac
done

# Full image reference
IMAGE="${REGISTRY}/${REPOSITORY}"

GIT_HASH=$(get_git_hash)
GIT_TAG=$(get_git_tag)
TAGS=("latest" "$GIT_HASH")

if [ "$VERSION" ]; then
    TAGS+=("$VERSION")
fi

if [ "$GIT_TAG" ]; then
    TAGS+=("$GIT_TAG")
fi

# Check if docker is installed
if ! command -v docker &> /dev/null; then
    error_log "Docker is not installed"
    exit 1
fi

# Print banner
echo "🐳 Docker Build Script 🏗️"
echo "=========================="

# Start the build process
log "🚀" "Building image: $IMAGE"
log "🏷️" "Tags: ${TAGS[*]}"
log "📄" "Using Dockerfile: $DOCKERFILE"
log "📁" "Build context: $BUILD_CONTEXT"

# Build the image with the first tag
PRIMARY_TAG="${IMAGE}:${TAGS[0]}"

if docker build -t "$PRIMARY_TAG" -f "$DOCKERFILE" "$BUILD_CONTEXT"; then
    log "✅" "Successfully built image: $PRIMARY_TAG"
    
    # Add additional tags to the image
    for ((i=1; i<${#TAGS[@]}; i++)); do
        tag="${TAGS[$i]}"
        docker tag "$PRIMARY_TAG" "${IMAGE}:${tag}"
        log "🏷️" "Tagged image with: ${tag}"
    done
    
    # Push all tagged images to registry
    log "⬆️" "Pushing images to registry"
    
    for tag in "${TAGS[@]}"; do
        log "⏳" "Pushing ${IMAGE}:${tag}"
        if docker push "${IMAGE}:${tag}"; then
            log "✅" "Successfully pushed ${IMAGE}:${tag}"
        else
            error_log "Failed to push ${IMAGE}:${tag}"
            exit 1
        fi
    done
    
    echo
    log "✅" "Successfully built and pushed all images"
    log "📦" "Image: $IMAGE"
    log "🏷️" "Tags: ${TAGS[*]}"
else
    echo
    error_log "Build failed"
    exit 1
fi