#!/bin/bash

# Exit on any error
set -e

# Default values
REGISTRY="192.168.1.241:5000"
REPOSITORY="mismatch"
NAMESPACE="mismatch-system"
RELEASE_NAME="mismatch"
HELM_CHART_PATH="./helm"

# Function to display usage
usage() {
    echo "📋 Usage: $0 [-v VERSION] [-r REGISTRY] [-n NAMESPACE] [-c HELM_CHART_PATH]"
    echo "  -v: Version tag to deploy (default: git tag or commit hash)"
    echo "  -r: Registry URI (default: ${REGISTRY})"
    echo "  -n: Kubernetes namespace (default: ${NAMESPACE})"
    echo "  -c: Path to Helm chart directory (default: ${HELM_CHART_PATH})"
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

while getopts "v:r:n:c:" opt; do
    case $opt in
        v) VERSION="$OPTARG";;
        r) REGISTRY="$OPTARG";;
        n) NAMESPACE="$OPTARG";;
        c) HELM_CHART_PATH="$OPTARG";;
        ?) usage;;
    esac
done

if [ -z "$VERSION" ]; then
    VERSION=$(get_version_identifier)
fi

# Check if helm is installed
if ! command -v helm &> /dev/null; then
    error_log "Helm is not installed"
    exit 1
fi

# Check if helm chart exists
if [ ! -d "$HELM_CHART_PATH" ]; then
    error_log "Helm chart directory not found: $HELM_CHART_PATH"
    exit 1
fi

# Log deployment details
log "🚀" "Deploying Helm chart..."
log "🏷️" "Version: ${VERSION}"
log "🔄" "Registry: ${REGISTRY}"
log "📦" "Namespace: ${NAMESPACE}"
log "📁" "Helm chart: ${HELM_CHART_PATH}"

# Deploy using Helm
if helm upgrade --install \
    ${RELEASE_NAME} \
    ${HELM_CHART_PATH} \
    --namespace ${NAMESPACE} \
    --create-namespace \
    --set image.tag=${VERSION} \
    --set image.repository=${REGISTRY}/${REPOSITORY} \
    --set appVersion="${VERSION}"; then
    
    echo
    log "✅" "Helm chart deployment complete"
else
    echo
    error_log "Deployment failed"
    exit 1
fi