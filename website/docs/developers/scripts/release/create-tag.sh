#!/bin/bash
set -e

if [ -z "$1" ] || [ -z "$2" ]; then
    echo "Error: TAG and MESSAGE are required."
    echo "Usage: $0 <tag> <message>"
    echo "Example: $0 v1.2.3 'Release v1.2.3'"
    exit 1
fi

TAG="$1"
MESSAGE="$2"

echo "Creating annotated tag $TAG..."
git tag -a "$TAG" -m "$MESSAGE"

echo "Pushing tag $TAG..."
git push origin "$TAG"

echo "Tag $TAG created and pushed successfully!"