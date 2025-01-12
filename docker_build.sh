#!/bin/bash

# Exit on any error
set -e

# Default values
IMAGE_NAME="mismatch"
PLATFORMS="linux/amd64,linux/arm64"
DOCKERFILE="Dockerfile"
BUILD_CONTEXT="."
AWS_REGION="us-east-1"
ECR_REPO_URI="767828732964.dkr.ecr.us-east-1.amazonaws.com"

# Function to display usage
usage() {
    echo "📋 Usage: $0 -i IMAGE_NAME -r ECR_REPO_URI [-v VERSION] [-p PLATFORMS] [-f DOCKERFILE] [-c BUILD_CONTEXT] [-a AWS_REGION]"
    echo "  -i: Image name (default: ${mismatch})"
    echo "  -r: ECR repository URI (default: ${ECR_REPO_URI})"
    echo "  -v: Additional version tag (default: git commit hash)"
    echo "  -p: Platforms to build for (default: linux/amd64,linux/arm64)"
    echo "  -f: Dockerfile path (default: Dockerfile)"
    echo "  -c: Build context path (default: current directory)"
    echo "  -a: AWS region (default: us-east-1)"
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

# Parse command line arguments
while getopts "i:r:v:p:f:c:a:" opt; do
    case $opt in
        i) IMAGE_NAME="$OPTARG";;
        r) ECR_REPO_URI="$OPTARG";;
        v) VERSION="$OPTARG";;
        p) PLATFORMS="$OPTARG";;
        f) DOCKERFILE="$OPTARG";;
        c) BUILD_CONTEXT="$OPTARG";;
        a) AWS_REGION="$OPTARG";;
        ?) usage;;
    esac
done

# Check if image name and ECR repository URI are provided
if [ -z "$IMAGE_NAME" ] || [ -z "$ECR_REPO_URI" ]; then
    error_log "Image name and ECR repository URI are required"
    usage
fi

GIT_HASH=$(get_git_hash)

if [ "$VERSION" ]; then
    TAGS=("latest" "$VERSION" "$GIT_HASH")
else
    TAGS=("latest" "$GIT_HASH")
fi

for tag in "${TAGS[@]}"; do
    TAG_ARGS="--tag ${ECR_REPO_URI}/${IMAGE_NAME}:${tag} $TAG_ARGS"
done

# Check if docker is installed
if ! command -v docker &> /dev/null; then
    error_log "Docker is not installed"
    exit 1
fi

# Check if buildx is installed
if ! docker buildx version &> /dev/null; then
    error_log "Docker buildx is not installed"
    exit 1
fi

# Check if aws cli is installed
if ! command -v aws &> /dev/null; then
    error_log "AWS CLI is not installed"
    exit 1
fi

# Authenticate Docker to the ECR registry
log "🔑" "Authenticating Docker to ECR"
aws ecr get-login-password --region "$AWS_REGION" | docker login --username AWS --password-stdin "$ECR_REPO_URI"

# Print banner
echo "🐳 Docker Multi-Architecture Build Script 🏗️"
echo "============================================"

# Create a new builder instance if it doesn't exist
if ! docker buildx inspect multiarch-builder &> /dev/null; then
    log "🔧" "Creating new buildx builder instance"
    docker buildx create --name multiarch-builder --driver docker-container --bootstrap
fi

# Use the builder
log "🔄" "Switching to multiarch builder"
docker buildx use multiarch-builder

# Start the build process
log "🚀" "Starting multi-architecture build for $IMAGE_NAME"
log "🏷️" "Tags: ${TAGS[*]}"
log "💻" "Building for platforms: $PLATFORMS"
log "📄" "Using Dockerfile: $DOCKERFILE"
log "📁" "Build context: $BUILD_CONTEXT"

# Build and push the images
if docker buildx build \
    --platform "$PLATFORMS" \
    $TAG_ARGS \
    --file "$DOCKERFILE" \
    --push \
    "$BUILD_CONTEXT"; then
    
    echo
    log "✅" "Successfully built and pushed multi-architecture image"
    log "📦" "Image: $IMAGE_NAME"
    log "🏷️" "Tags: ${TAGS[*]}"
    log "🎯" "Platforms: $PLATFORMS"
else
    echo
    error_log "Build failed"
    exit 1
fi
