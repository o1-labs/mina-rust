# How to build and launch a node from source

## Building from Source

For detailed instructions on how to build OpenMina from source, including system
dependencies, toolchain setup, and platform-specific instructions, please refer
to the [Getting Started for Developers](../developers/getting-started.mdx)
guide.

The developer guide includes:

- Tested setup instructions for Ubuntu 22.04, Ubuntu 24.04, and macOS
- Complete toolchain installation (Rust, Node.js, Docker, etc.)
- Build verification and testing procedures
- Environment configuration

## Running the Node

Once you have built OpenMina following the developer guide, you can run the node
using the provided Makefile targets:

### Available Make Targets

#### Basic Node

```bash
# Run a basic node (defaults: NETWORK=devnet, VERBOSITY=info)
make run-node

# Run with custom network and verbosity level
make run-node NETWORK=mainnet VERBOSITY=debug
```

#### Archive Node

```bash
# Run an archive node with local storage
make run-archive

# Run archive node on mainnet
make run-archive NETWORK=mainnet
```

For comprehensive archive node setup including different storage options,
database configuration, and Docker Compose deployment, see the detailed
[Archive Node](./archive-node.md) documentation.

### Configuration Variables

The Makefile supports the following configuration variables that can be
overridden:

- `NETWORK` - Network to connect to (default: `devnet`, options: `devnet`,
  `mainnet`)
- `VERBOSITY` - Logging verbosity level (default: `info`, options: `error`,
  `warn`, `info`, `debug`, `trace`)

### Advanced Configuration and CLI Parameters

For detailed information about all available command-line parameters and
advanced configuration options, please refer to the
[Rust API Documentation](https://o1-labs.github.io/openmina/api-docs/cli/commands/node/struct.Node.html)
which contains comprehensive documentation for all supported parameters
including:

- Network and connection options
- Block producer configuration
- Archive node settings
- Logging and debugging options
- P2P networking parameters
- Snarker configuration
- And more

You can also get help directly from the command line:

```bash
./target/release/openmina node --help
```
