# Run Archive Node on Devnet

This guide is intended for setting up archive nodes on **Mina Devnet** only. Do
not use this guide for Mina Mainnet

## Archive Mode Configuration

We start archive mode in Mina Rust by setting one of the following flags along
with their associated environment variables:

### Archiver Process (`--archive-archiver-process`)

Stores blocks in a database by receiving them directly from the Mina Rust node

**Required Environment Variables**:

- `MINA_ARCHIVE_ADDRESS`: Network address for the archiver service

### Local Storage (`--archive-local-storage`)

Stores blocks in the local filesystem

**Required Environment Variables**:

- (None)

**Optional Environment Variables**:

- `MINA_ARCHIVE_LOCAL_STORAGE_PATH`: Custom path for block storage (default:
  ~/.mina/archive-precomputed)

### GCP Storage (`--archive-gcp-storage`)

Uploads blocks to a Google Cloud Platform bucket

**Required Environment Variables**:

- `GCP_CREDENTIALS_JSON`: Service account credentials JSON
- `GCP_BUCKET_NAME`: Target storage bucket name

### AWS Storage (`--archive-aws-storage`)

Uploads blocks to an AWS S3 bucket

**Required Environment Variables**:

- `AWS_ACCESS_KEY_ID`: IAM user access key
- `AWS_SECRET_ACCESS_KEY`: IAM user secret key
- `AWS_DEFAULT_REGION`: AWS region name
- `AWS_SESSION_TOKEN`: Temporary session token for temporary credentials
- `MINA_AWS_BUCKET_NAME`: Target S3 bucket name

## Redundancy

The archive mode is designed to be redundant. We can combine the flags to have
multiple options running simultaneously.

## Prerequisites

Ensure Docker and Docker Compose are installed on your system -
[Docker Installation Guide](../appendix/docker-installation)

## Docker compose setup (with archiver process)

The compose file sets up a PG database, the archiver process and the Mina Rust
node. The archiver process is responsible for storing the blocks in the database
by receiving the blocks from the Mina Rust node.

See
[docker-compose.archive.devnet.yml](https://github.com/o1-labs/mina-rust/blob/develop/docker-compose.archive.devnet.yml)
for more details.

### Starting the setup

```bash
docker compose -f docker-compose.archive.devnet.yml up -d
```
