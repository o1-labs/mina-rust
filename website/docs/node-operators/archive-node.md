# Run Archive Node

This guide is intended for setting up archive nodes on **Mina Devnet** only. Do
not use this guide for Mina Mainnet until necessary security audits are
complete.

---

## Prerequisites

Ensure Docker and Docker Compose are installed on your system -
[Docker Installation Guide](../appendix/docker-installation)

## Download & Start the Archive Node

1. **Download the Latest Release**
   - Visit the
     [Mina Rust Releases](https://github.com/o1-labs/mina-rust/releases)
   - Download the latest `mina-rust-vX.Y.Z-docker-compose.zip`
   - Extract the Files:

     ```bash
     unzip mina-rust-vX.Y.Z-docker-compose.zip
     cd mina-rust-vX.Y.Z-docker-compose
     ```

2. **Launch Archive Node**

   The archive node setup includes a PostgreSQL database, the archiver process,
   and the Mina Rust node. The archiver process stores blocks in the database by
   receiving them from the Mina Rust node.

   ```bash
   docker compose -f docker-compose.archive.devnet.yml up -d --pull always
   ```

   **Configuration Options:**
   - `MINA_RUST_TAG` - Docker image tag for the mina-rust node (default:
     `latest`)
   - `POSTGRES_PASSWORD` - Database password for PostgreSQL
   - `PG_PORT` - PostgreSQL port (default: `5432`)
   - `PG_DB` - Database name (default: `archive`)

   **Examples with different versions:**

   ```bash
   # Use specific version (recommended for production)
   env MINA_RUST_TAG="v1.4.2" \
   docker compose -f docker-compose.archive.devnet.yml up -d --pull always

   # Use development version (latest features, may be unstable)
   env MINA_RUST_TAG="develop" \
   docker compose -f docker-compose.archive.devnet.yml up -d --pull always
   ```

3. **Monitor the Archive Node**

   The archive node will be accessible at:
   - **Archive API**: http://localhost:3086
   - **Node API**: http://localhost:3000

## Node Parameters Reference

For a complete list of all available archive node parameters and configuration
options, see the
[Mina Rust API Documentation](https://o1-labs.github.io/mina-rust/api-docs/mina_cli/commands/node/struct.NodeArgs.html).
This includes detailed descriptions of:

- **Archive configuration flags**: `--archive-archiver-process`,
  `--archive-local-storage`, `--archive-gcp-storage`, `--archive-aws-storage`
- **Network settings**: `--libp2p-*`, `--network`, `--port`
- **Logging and debugging options**: `--verbosity`, `--log-*`
- **Performance tuning parameters**: Connection limits, timeouts, etc.
- **Security and validation settings**: Key management, validation options

You can also view available parameters by running:

```bash
# View all node subcommand options
mina node --help

# View specific archive-related options
mina node --help | grep -A 10 -B 2 archive
```

The source code documentation can be found in
[`cli/src/commands/node/mod.rs`](https://github.com/o1-labs/mina-rust/blob/develop/cli/src/commands/node/mod.rs)
which contains comprehensive examples and parameter descriptions for all archive
node configurations.

## Using Make Command

As an alternative to Docker Compose, you can run the archive node directly using
the Makefile target. This method requires building from source.

### Prerequisites

- Rust toolchain installed
- Git repository cloned and accessible
- PostgreSQL database running and configured

### Archive Mode Configuration

Mina Rust supports multiple archive modes that can be run simultaneously for
redundancy:

**Archiver Process (`--archive-archiver-process`)**

Stores blocks in a database by receiving them directly from the Mina Rust node.

**Required Environment Variables:**

- `MINA_ARCHIVE_ADDRESS`: Network address for the archiver service

**Local Storage (`--archive-local-storage`)**

Stores blocks in the local filesystem.

**Optional Environment Variables:**

- `MINA_ARCHIVE_LOCAL_STORAGE_PATH`: Custom path for block storage (default:
  `~/.mina/archive-precomputed`)

**GCP Storage (`--archive-gcp-storage`)**

Uploads blocks to a Google Cloud Platform bucket.

**Required Environment Variables:**

- `GCP_CREDENTIALS_JSON`: Service account credentials JSON
- `GCP_BUCKET_NAME`: Target storage bucket name

**AWS Storage (`--archive-aws-storage`)**

Uploads blocks to an AWS S3 bucket.

**Required Environment Variables:**

- `AWS_ACCESS_KEY_ID`: IAM user access key
- `AWS_SECRET_ACCESS_KEY`: IAM user secret key
- `AWS_DEFAULT_REGION`: AWS region name
- `AWS_SESSION_TOKEN`: Temporary session token for temporary credentials
- `MINA_AWS_BUCKET_NAME`: Target S3 bucket name

### Setup and Run

1. **Run Archive Node with Archiver Process**

   For devnet (default):

   ```bash
   MINA_ARCHIVE_ADDRESS="http://localhost:3086" \
   make run-node NETWORK=devnet -- --archive-archiver-process
   ```

   For mainnet (when supported):

   ```bash
   MINA_ARCHIVE_ADDRESS="http://localhost:3086" \
   make run-node NETWORK=mainnet -- --archive-archiver-process
   ```

2. **Run with Multiple Archive Modes (Redundancy)**

   You can combine multiple archive modes for redundancy:

   ```bash
   # Archive to both database and local storage
   MINA_ARCHIVE_ADDRESS="http://localhost:3086" \
   MINA_ARCHIVE_LOCAL_STORAGE_PATH="/path/to/archive" \
   make run-node NETWORK=devnet -- \
     --archive-archiver-process \
     --archive-local-storage
   ```

   ```bash
   # Archive to database, local storage, and AWS S3
   MINA_ARCHIVE_ADDRESS="http://localhost:3086" \
   MINA_ARCHIVE_LOCAL_STORAGE_PATH="/path/to/archive" \
   AWS_ACCESS_KEY_ID="your-access-key" \
   AWS_SECRET_ACCESS_KEY="your-secret-key" \
   AWS_DEFAULT_REGION="us-west-2" \
   MINA_AWS_BUCKET_NAME="your-bucket-name" \
   make run-node NETWORK=devnet -- \
     --archive-archiver-process \
     --archive-local-storage \
     --archive-aws-storage
   ```

3. **Monitor the Node**

   The node will start and listen on port 3000. You can monitor its status by
   checking the console output or connecting a frontend dashboard.

### Access Logs

Logs are stored in the working directory with filenames like
`mina.log.2024-10-14`, `mina.log.2024-10-15`, etc.

### Provide Feedback

Collect logs and report issues on the
[rust-node-testing](https://discord.com/channels/484437221055922177/1290662938734231552)
Discord channel. Include reproduction steps if possible.
