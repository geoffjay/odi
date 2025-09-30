# ODI Docker Test Environment

This directory contains Docker containers for testing ODI's remote operations functionality.

## Available Services

### SSH Server
- **Port**: 2222
- **Username**: odiuser
- **Password**: odipass
- **Connection**: `ssh -p 2222 odiuser@localhost`
- **Repository Path**: `/repos` (mounted from `.docker/ssh-repos`)

### HTTP Server (Simple Python)
- **Port**: 8090
- **URL**: http://localhost:8090/
- **Repository Path**: `/var/www/repos` (mounted from `.docker/https-repos`)
- **Test Repository**: http://localhost:8090/test-repo.odi/

### Git HTTP Server
- **Port**: 9418 
- **URL**: http://localhost:9418/
- **Repository Path**: `/git-repos` (mounted from `.docker/git-repos`)
- **Test Repository**: http://localhost:9418/test-repo.git/

## Usage

Start all services:
```bash
docker compose up -d
```

Stop all services:
```bash
docker compose down
```

Check service status:
```bash
docker compose ps
```

View logs:
```bash
docker compose logs [service-name]
```

## Testing Remote Operations

These containers provide a local testing environment for ODI's remote synchronization features:

1. **SSH Protocol Testing**: Use the SSH server to test `odi remote add ssh://...` operations
2. **HTTP Protocol Testing**: Use the HTTP servers to test `odi remote add http://...` operations  
3. **Manual Testing**: Copy test repositories to the mounted volumes to simulate remote repositories

## Directory Structure

- `.docker/ssh-repos/` - SSH server repository storage
- `.docker/https-repos/` - HTTP server repository storage  
- `.docker/git-repos/` - Git server repository storage
- `.docker/ssh-keys/` - SSH key storage (for key-based auth)
- `.docker/ssl-certs/` - SSL certificate storage (for HTTPS)