# Test Data for ODI Remote Operations

This directory contains test data and configurations for testing ODI remote operations using Docker containers.

## Directory Structure

- `ssh-repos/`: Repository data for SSH server testing
- `ssh-keys/`: SSH keys for authentication testing
- `https-repos/`: Repository data for HTTPS server testing  
- `ssl-certs/`: SSL certificates for HTTPS server
- `git-repos/`: Git repositories for comparison testing

## Usage

1. Start the test containers:
   ```bash
   docker-compose up -d
   ```

2. Test SSH connectivity:
   ```bash
   ssh -p 2222 odiuser@localhost
   # Password: odipass
   ```

3. Test HTTPS connectivity:
   ```bash
   curl -k https://localhost:8443/
   curl http://localhost:8080/
   ```

4. Stop the containers:
   ```bash
   docker-compose down
   ```

## Container Services

- **odi-ssh-server** (port 2222): SSH server for testing `ssh://` protocol operations
- **odi-https-server** (ports 8080/8443): Apache server for testing `https://` and `http://` protocol operations  
- **git-server** (port 9418): Git daemon for comparison testing

## Testing Remote Operations

Once containers are running, you can test ODI remote operations:

```bash
# SSH remote operations
odi remote add origin ssh://odiuser@localhost:2222/repos/test-repo.odi

# HTTPS remote operations  
odi remote add origin https://localhost:8443/test-repo.odi
odi remote add origin http://localhost:8080/test-repo.odi
```

## Security Note

These containers are for testing only and use:
- Default passwords (`odipass`)
- Self-signed certificates
- Permissive configurations

Do not use these configurations in production environments.