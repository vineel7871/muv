#!/bin/bash

# Script to create and initialize the Homebrew tap repository

set -e

REPO_NAME="homebrew-muv"
GITHUB_USER="vineel7871"

echo "🍺 Setting up Homebrew tap for muv..."

# Check if gh CLI is installed
if ! command -v gh &> /dev/null; then
    echo "❌ GitHub CLI (gh) is required but not installed."
    echo "Install it with: brew install gh"
    exit 1
fi

# Check if user is authenticated
if ! gh auth status &> /dev/null; then
    echo "❌ Please authenticate with GitHub CLI first:"
    echo "gh auth login"
    exit 1
fi

echo "📁 Creating GitHub repository: ${GITHUB_USER}/${REPO_NAME}"

# Create the repository
gh repo create "${REPO_NAME}" --public --description "Homebrew tap for muv - Global environment management tool using uv"

echo "📂 Cloning and setting up local repository..."

# Clone and set up the repository
git clone "https://github.com/${GITHUB_USER}/${REPO_NAME}.git"
cd "${REPO_NAME}"

# Create directory structure
mkdir -p Formula

# Create README
cat > README.md << 'EOF'
# Homebrew Tap for muv

This is the official Homebrew tap for [muv](https://github.com/vineel7871/muv), a global environment management tool using uv.

## Installation

```bash
brew tap vineel7871/muv
brew install muv
```

## Usage

After installation, you can use muv:

```bash
muv --help
```

For more information, visit the [main repository](https://github.com/vineel7871/muv).
EOF

# Initial commit
git add .
git commit -m "Initial commit: Set up Homebrew tap for muv"
git push origin main

echo "✅ Homebrew tap repository created successfully!"
echo ""
echo "Next steps:"
echo "1. Go to https://github.com/${GITHUB_USER}/muv/settings/secrets/actions"
echo "2. Add HOMEBREW_TAP_TOKEN secret with a GitHub Personal Access Token"
echo "3. Create a new release to test the automated formula update"
echo ""
echo "Users can now install muv with:"
echo "  brew tap ${GITHUB_USER}/muv"
echo "  brew install muv"