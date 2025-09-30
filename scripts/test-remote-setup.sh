#!/bin/bash

# ODI Remote Testing Setup Script
# Sets up test repositories in the Docker containers for remote operation testing

set -e

echo "🐳 Setting up ODI remote testing environment..."

# Check if Docker containers are running
if ! docker compose ps | grep -q "odi-ssh-server.*Up"; then
    echo "❌ SSH server container is not running. Please run 'docker compose up -d' first."
    exit 1
fi

if ! docker compose ps | grep -q "odi-https-server.*Up"; then
    echo "❌ HTTPS server container is not running. Please run 'docker compose up -d' first."
    exit 1
fi

echo "✅ Docker containers are running"

# Create test repositories in SSH server
echo "📁 Creating test repositories in SSH server..."
docker exec odi-ssh-server sh -c '
    mkdir -p /repos/test-project.odi
    cd /repos/test-project.odi
    echo "Test ODI project for SSH remote testing" > README.txt
    
    # Create a sample .odi structure
    mkdir -p .odi/objects .odi/refs
    echo "ref: refs/heads/main" > .odi/HEAD
    echo "Test project initialized for SSH testing" > .odi/description
    
    chown -R odiuser:odiuser /repos/test-project.odi
'

# Create test repositories in HTTPS server  
echo "📁 Creating test repositories in HTTPS server..."
docker exec odi-https-server sh -c '
    mkdir -p /var/www/repos/test-project.odi
    cd /var/www/repos/test-project.odi
    echo "Test ODI project for HTTPS remote testing" > README.txt
    
    # Create a sample .odi structure
    mkdir -p .odi/objects .odi/refs
    echo "ref: refs/heads/main" > .odi/HEAD
    echo "Test project initialized for HTTPS testing" > .odi/description
    
    chown -R apache:apache /var/www/repos/test-project.odi
'

# Generate SSH key for testing (if it doesn't exist)
if [ ! -f ".docker/ssh-keys/odi_test_key" ]; then
    echo "🔑 Generating SSH test key..."
    ssh-keygen -t rsa -b 2048 -f .docker/ssh-keys/odi_test_key -N "" -C "odi-test@localhost"
    
    # Add public key to SSH server
    docker exec odi-ssh-server sh -c '
        mkdir -p /home/odiuser/.ssh
        cat > /home/odiuser/.ssh/authorized_keys
        chmod 600 /home/odiuser/.ssh/authorized_keys
        chown odiuser:odiuser /home/odiuser/.ssh/authorized_keys
    ' < .docker/ssh-keys/odi_test_key.pub
fi

echo "✅ Test repositories created successfully!"

echo "
🧪 Test Environment Ready!

SSH Server:
  - Host: localhost:2222
  - User: odiuser / Password: odipass
  - Test repo: ssh://odiuser@localhost:2222/repos/test-project.odi
  - SSH key: .docker/ssh-keys/odi_test_key

HTTPS Server:
  - HTTP: http://localhost:8080/test-project.odi
  - HTTPS: https://localhost:8443/test-project.odi (self-signed cert)
  
You can now test ODI remote operations:
  odi remote add origin ssh://odiuser@localhost:2222/repos/test-project.odi
  odi remote add origin https://localhost:8443/test-project.odi
"
