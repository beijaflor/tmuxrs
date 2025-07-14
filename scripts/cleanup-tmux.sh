#!/bin/bash
# Cleanup script for tmux sessions after testing

echo "🧹 Cleaning up tmux sessions..."

# List current sessions before cleanup
echo "Current tmux sessions:"
tmux list-sessions 2>/dev/null || echo "No tmux sessions found"

# Kill all tmux sessions
echo "Killing all tmux sessions..."
tmux kill-server 2>/dev/null || true

# Verify cleanup
echo "Checking for remaining sessions..."
if tmux list-sessions 2>/dev/null; then
    echo "⚠️  Warning: Some tmux sessions still exist"
    exit 1
else
    echo "✅ All tmux sessions cleaned up successfully"
fi

# Clean up any tmux socket files
echo "Cleaning up tmux socket files..."
rm -f /tmp/tmux-*/default 2>/dev/null || true
rm -f /tmp/tmux-*/* 2>/dev/null || true

echo "🎉 Cleanup complete!"