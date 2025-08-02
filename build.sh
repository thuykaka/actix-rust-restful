#!/bin/bash

# Build script for Rust application
# Usage: ./build.sh [dockerfile] [tag]

set -e

# Default values
DOCKERFILE=${1:-"Dockerfile"}
TAG=${2:-"actix-rust-restful:latest"}

echo "🐳 Building Docker image..."
echo "📁 Dockerfile: $DOCKERFILE"
echo "🏷️  Tag: $TAG"

# Check if Dockerfile exists
if [ ! -f "$DOCKERFILE" ]; then
    echo "❌ Error: $DOCKERFILE not found!"
    exit 1
fi

# Build the image
echo "🔨 Building with docker build..."
docker build -f "$DOCKERFILE" -t "$TAG" .

# Check build result
if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "📊 Image info:"
    docker images "$TAG"
else
    echo "❌ Build failed!"
    exit 1
fi

echo "🚀 Ready to run with: docker run -p 8080:8080 $TAG" 