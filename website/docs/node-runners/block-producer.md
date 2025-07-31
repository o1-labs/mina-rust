# Run Block Producing Node on Devnet

This guide is intended for setting up block producer nodes on **Mina Devnet**
only. Do not use this guide for Mina Mainnet until necessary security audits are
complete.

---

## Prerequisites

Ensure Docker and Docker Compose are installed on your system -
[Docker Installation Guide](./docker-installation.md)

## Download & Start the Node

1. **Download the Latest Release**

- Visit the [Open Mina Releases](https://github.com/openmina/openmina/releases)
- Download the latest `openmina-vX.Y.Z-docker-compose.zip`
- Extract the Files:

  ```bash
  unzip openmina-vX.Y.Z-docker-compose.zip
  cd openmina-vX.Y.Z-docker-compose
  mkdir openmina-workdir
  ```

2. **Prepare Your Keys**

   [Docker Compose](https://github.com/o1-labs/openmina/blob/develop/docker-compose.block-producer.yml)
   references `openmina-workdir`. It stores a private key and logs for block
   production. Place your block producer's private key into the
   `openmina-workdir` directory and name it `producer-key`:

   ```bash
   cp /path/to/your/private_key openmina-workdir/producer-key
   ```

   Replace `/path/to/your/private_key` with the actual path to your private key
   file.

3. **Launch Block Producer**

   Use `MINA_PRIVKEY_PASS` to set the private key password. Optionally, use
   `COINBASE_RECEIVER` to set a different coinbase receiver:

   ```bash
   env COINBASE_RECEIVER="YourWalletAddress" MINA_PRIVKEY_PASS="YourPassword" \
   docker compose -f docker-compose.block-producer.yml up -d --pull always
   ```

   Optional parameters:

   `OPENMINA_LIBP2P_EXTERNAL_IP` Sets your node’s external IP address to help
   other nodes find it.

   `OPENMINA_LIBP2P_PORT` Sets the port for Libp2p communication.

4. **Go to Dashboard**

   Visit [http://localhost:8070](http://localhost:8070) to
   [monitor sync](http://localhost:8070/dashboard) and
   [block production](http://localhost:8070/block-production).

## Alternative: Using Make Command

As an alternative to Docker Compose, you can run the block producer directly
using the Makefile target. This method requires building from source.

### Prerequisites

- Rust toolchain installed
- Git repository cloned and accessible

### Setup and Run

1. **Prepare Your Keys**

   You have two options for setting up your producer key:

   **Option A: Generate a new key pair**

   ```bash
   make generate-block-producer-key
   ```

   This will create a new key pair and save the private key to
   `openmina-workdir/producer-key`.

   **Option B: Use an existing key**

   ```bash
   mkdir -p openmina-workdir
   cp /path/to/your/private_key openmina-workdir/producer-key
   ```

2. **Run Block Producer**

   For devnet (default):

   ```bash
   make run-block-producer-devnet COINBASE_RECEIVER="YourWalletAddress" \
   MINA_PRIVKEY_PASS="YourPassword"
   ```

   Or explicitly specify devnet:

   ```bash
   make run-block-producer NETWORK=devnet COINBASE_RECEIVER="YourWalletAddress" \
   MINA_PRIVKEY_PASS="YourPassword"
   ```

   For mainnet (when supported):

   ```bash
   make run-block-producer-mainnet COINBASE_RECEIVER="YourWalletAddress" \
   MINA_PRIVKEY_PASS="YourPassword"
   ```

   Optional parameters:
   - `OPENMINA_LIBP2P_EXTERNAL_IP` - Sets external IP address
   - `OPENMINA_LIBP2P_PORT` - Sets libp2p communication port
   - `PRODUCER_KEY` - Path to producer key (default:
     `./openmina-workdir/producer-key`)

   Example with all options:

   ```bash
   make run-block-producer-devnet \
     COINBASE_RECEIVER="YourWalletAddress" \
     MINA_PRIVKEY_PASS="YourPassword" \
     OPENMINA_LIBP2P_EXTERNAL_IP="1.2.3.4" \
     OPENMINA_LIBP2P_PORT="8302"
   ```

3. **Monitor the Node**

   The node will start and listen on port 3000. You can monitor its status by
   checking the console output or connecting a frontend dashboard.

### Access Logs

Logs are stored in `openmina-workdir` with filenames like
`openmina.log.2024-10-14`, `openmina.log.2024-10-15`, etc.

### Provide Feedback

Collect logs from `openmina-workdir` and report issues on the
[rust-node-testing](https://discord.com/channels/484437221055922177/1290662938734231552)
discord channel. Include reproduction steps if possible.
