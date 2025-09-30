#!/bin/bash

# Test Docker Setup Script
# Quick test to verify the containers work correctly

set -e

echo "🧪 Testing Docker setup for ODI remote operations..."

# Check if Docker is available
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed or not in PATH"
    exit 1
fi

if ! command -v docker compose &> /dev/null; then
    echo "❌ Docker Compose is not installed or not in PATH"
    exit 1
fi

echo "✅ Docker and Docker Compose are available"

# Start containers
echo "🚀 Starting test containers..."
docker compose up -d

# Wait for containers to be ready
echo "⏳ Waiting for containers to start..."
sleep 10

# Check if containers are running
if ! docker compose ps | grep -q "odi-ssh-server.*Up"; then
    echo "❌ SSH server container failed to start"
    docker compose logs odi-ssh-server
    exit 1
fi

if ! docker compose ps | grep -q "odi-https-server.*Up"; then
    echo "❌ HTTPS server container failed to start"
    docker compose logs odi-https-server
    exit 1
fi

echo "✅ All containers are running"

# Test SSH connectivity
echo "🔐 Testing SSH connectivity..."
if docker exec odi-ssh-server ps aux | grep -q sshd; then
    echo "✅ SSH daemon is running"
else
    echo "❌ SSH daemon is not running"
    exit 1
fi

# Test HTTPS connectivity  
echo "🌐 Testing HTTPS connectivity..."
if docker exec odi-https-server ps aux | grep -q httpd; then
    echo "✅ Apache is running"
else
    echo "❌ Apache is not running"
    exit 1
fi

# Test port accessibility
echo "🔌 Testing port accessibility..."
if nc -z localhost 2222 2>/dev/null; then
    echo "✅ SSH port 2222 is accessible"
else
    echo "⚠️ SSH port 2222 may not be ready yet"
fi

if nc -z localhost 8080 2>/dev/null; then
    echo "✅ HTTP port 8080 is accessible"
else
    echo "⚠️ HTTP port 8080 may not be ready yet"
fi

if nc -z localhost 8443 2>/dev/null; then
    echo "✅ HTTPS port 8443 is accessible"
else
    echo "⚠️ HTTPS port 8443 may not be ready yet"
fi

echo "
🎉 Docker setup test completed!

Next steps:
1. Run ./scripts/test-remote-setup.sh to set up test repositories
2. Test ODI remote operations with the containers
3. Run ./scripts/test-remote-cleanup.sh when done

Container status:
$(docker compose ps)
"
