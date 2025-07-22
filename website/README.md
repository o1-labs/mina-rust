# OpenMina Documentation Website

This directory contains the Docusaurus-based documentation website for OpenMina.

## Quick Start

### Development
```bash
# From project root
make docs-serve
# Or directly:
cd website && npm start
```

### Build
```bash
# From project root  
make docs-build
# Or directly:
cd website && npm run build
```

### Serve Built Site
```bash
# From project root
make docs-build-serve
# Or directly:
cd website && npm run serve
```

## Structure

### User-Focused Documentation
The documentation is organized around three main user personas:

- **Node Runners** (`docs/node-runners/`) - Installation, configuration, and operation guides
- **Developers** (`docs/developers/`) - Architecture, codebase understanding, and contribution guides  
- **Researchers** (`docs/researchers/`) - Protocol details, cryptography, and research materials

### Configuration
- `docusaurus.config.ts` - Main configuration file
- `sidebars.ts` - Navigation structure for each user type
- `src/pages/index.tsx` - Homepage
- `src/components/` - Custom React components

## Adding Documentation

### New Pages
1. Create markdown files in the appropriate directory (`docs/node-runners/`, `docs/developers/`, or `docs/researchers/`)
2. Add frontmatter with title, description, and sidebar position:
   ```yaml
   ---
   sidebar_position: 1
   title: Page Title
   description: Brief description for SEO
   ---
   ```
3. Update `sidebars.ts` to include the new page in navigation

### Editing Content
Simply edit the markdown files - developers only need to modify markdown files, not the Docusaurus configuration.

## Versioning

The site supports versioning with the current version labeled as "develop". When tags are created, new documentation versions will be automatically deployed.

## Available Commands

From project root using Makefile:
- `make docs-install` - Install dependencies
- `make docs-build` - Build the website
- `make docs-serve` - Start development server  
- `make docs-build-serve` - Build and serve locally
- `make docs-clean` - Clean build artifacts
