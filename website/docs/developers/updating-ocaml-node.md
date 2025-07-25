---
sidebar_position: 4
---

# Updating the OCaml Node

When a new Mina Protocol release becomes available, you need to update the OCaml
node references in OpenMina. The OCaml node releases are used to verify that the
current OpenMina version maintains compatibility with the official Mina Protocol
implementation through end-to-end and scenario testing. This guide walks through
the process based on the workflow used in
[PR #1236](https://github.com/o1-labs/openmina/pull/1236).

## 1. Check for New Releases

Visit the
[Mina Protocol releases page](https://github.com/MinaProtocol/mina/releases/) to
find the latest release version and associated container image tags.

For example, release `3.2.0-alpha1` provides updated container images and
configuration files. You'll need:

- The new version number (e.g., `3.2.0-alpha1`)
- The 8-character configuration hash from the release page

## 2. Automated Update with Script

OpenMina provides an automation script to handle the bulk of the update process:

```bash
./website/docs/developers/scripts/update-ocaml-node.sh <old_hash> <new_hash> <old_version> <new_version>
```

**Example usage:**

For example, to update from `3.2.0-alpha1` to `3.2.0-beta1` (as done in commit
[31caeee6](https://github.com/o1-labs/openmina/commit/31caeee6af7bf20b8578a23bf69718dbe68fe5cc)):

```bash
./website/docs/developers/scripts/update-ocaml-node.sh 7f94ae0b 978866cd "3.2.0-alpha1" "3.2.0-beta1"
```

**Parameters:**

- `old_hash`: Current 8-character config hash
- `new_hash`: New 8-character config hash from the release
- `old_version`: Current version (e.g., `3.1.0`)
- `new_version`: New version (e.g., `3.2.0-alpha1`)

The script automatically updates:

- Configuration file references in testing code
- Docker image tags in CI workflows and compose files
- Version strings throughout the codebase

## 3. Manual Verification

After running the script, verify all references were updated correctly:

```bash
# Search for any remaining old references
git grep "gcr.io/o1labs-192920/mina-daemon"
git grep "config_<old_hash>"
git grep "<old_version>"
```

## 4. Verification Steps

After making updates:

1. **Build and Test**: Run the test suite to ensure compatibility
2. **Check References**: Verify all version references are updated consistently
3. **Configuration Validation**: Ensure new config files are properly referenced
4. **End-to-End Testing**: Run scenario tests to verify OpenMina compatibility
   with the updated OCaml node
5. **CI Pipeline**: Verify that automated testing passes with new versions

## 5. Commit Structure

Following the pattern from commit
[31caeee6](https://github.com/o1-labs/openmina/commit/31caeee6af7bf20b8578a23bf69718dbe68fe5cc):

1. **Main Update Commit**: "OCaml nodes: bump up to release [version]"
   - Updates 6 files: CI workflows, Docker compose, and testing configurations
   - Atomic change affecting all OCaml node references
2. **Changelog**: Add entry documenting the version bump if needed

## Related Resources

- [Mina Protocol Releases](https://github.com/MinaProtocol/mina/releases/)
- [OpenMina Architecture Documentation](./architecture.md)
- [Example PR #1236](https://github.com/o1-labs/openmina/pull/1236)
- [Node Runners Guide](../node-runners/getting-started.md)
