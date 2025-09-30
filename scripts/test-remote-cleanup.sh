#!/bin/bash

# ODI Remote Testing Cleanup Script
# Cleans up test data and stops Docker containers

set -e

echo "🧹 Cleaning up ODI remote testing environment..."

# Stop and remove containers
echo "🛑 Stopping Docker containers..."
docker compose down

# Clean up test data (optional - uncomment if you want to remove test data)
# echo "🗑️ Removing test data..."
# rm -rf .docker/ssh-keys/odi_test_key*
# rm -rf .docker/ssh-repos/*
# rm -rf .docker/https-repos/*

echo "✅ Cleanup complete!"

echo "
💡 To restart the test environment:
  docker compose up -d
  ./scripts/test-remote-setup.sh
"
