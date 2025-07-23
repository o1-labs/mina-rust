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
configuration files.

## 2. Update GCR Links in Code

Use `git grep` to find all Google Container Registry (GCR) references that need
updating:

```bash
# Search for GCR image references
git grep "gcr.io/o1labs-192920/mina-daemon"

# Search for any mina-daemon references
git grep "mina-daemon"
```

Update the image tags in the found files to match the new release version.
Typically this involves changing version strings in Docker Compose files,
testing configurations, and CI workflows.

## 3. Update Configuration Files

Update the configuration file hash to match the new release. The config files
with the pattern `config_[8-character-hash]` come from the OCaml node release
and need to be referenced in the OpenMina codebase.

The configuration hash corresponds to the genesis state and network parameters
for the specific release. You may need to:

- Update references to the config filename in code to match the new hash
- Download or reference the new config files from the OCaml node release
- Verify the new configuration is compatible with OpenMina

## 4. Example Workflow

Based on PR #1236, a typical update involves:

```bash
# 1. Find current references
git grep "gcr.io/o1labs-192920/mina-daemon"
git grep "3\.1\.0" # Search for old version numbers

# 2. Update image tags throughout the codebase
# Replace old tags with new release version

# 3. Search for config file references in code
git grep "config_"

# 4. Update any hardcoded version references
git grep -i "mina.*3\.1\.0" # Example for version 3.1.0
```

## 5. Verification Steps

After making updates:

1. **Build and Test**: Run the test suite to ensure compatibility
2. **Check References**: Verify all version references are updated consistently
3. **Configuration Validation**: Ensure new config files are properly referenced
4. **End-to-End Testing**: Run scenario tests to verify OpenMina compatibility
   with the updated OCaml node
5. **CI Pipeline**: Verify that automated testing passes with new versions

## 6. Commit Structure

Following the pattern from PR #1236:

1. **Main Update Commit**: "Update OCaml node dependencies to [version]"
2. **Changelog**: Add entry documenting the version bump
3. **Config Updates**: Separate commit for configuration file changes if needed

## Related Resources

- [Mina Protocol Releases](https://github.com/MinaProtocol/mina/releases/)
- [OpenMina Architecture Documentation](./architecture.md)
- [Example PR #1236](https://github.com/o1-labs/openmina/pull/1236)
- [Node Runners Guide](../node-runners/getting-started.md)
