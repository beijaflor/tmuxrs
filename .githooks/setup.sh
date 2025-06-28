#!/bin/sh
# Setup git hooks for tmuxrs

echo "Setting up git hooks..."
git config core.hooksPath .githooks
echo "✅ Git hooks configured successfully!"
echo "Pre-commit hook will run: fmt check, clippy, and tests"