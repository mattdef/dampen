#!/bin/bash
# Test script for hot-reload functionality

set -e

echo "=== Testing Hot-Reload Functionality ==="
echo ""

# Start the counter app in background
cd examples/counter
echo "Starting counter app..."
cargo run 2>&1 &
APP_PID=$!

# Give it time to start
sleep 3

echo ""
echo "Application started (PID: $APP_PID)"
echo "Watching logs..."

# Wait for initialization
sleep 1

echo ""
echo "Making a test change to window.dampen..."

# Make a small change to the file
sed -i 's/Counter: {count}/Count: {count}/g' src/ui/window.dampen

echo "Change made, waiting for hot-reload..."
sleep 2

# Revert the change
sed -i 's/Count: {count}/Counter: {count}/g' src/ui/window.dampen

echo "Reverted change"
sleep 1

# Kill the app
echo ""
echo "Stopping application..."
kill $APP_PID 2>/dev/null || true

echo "Test complete!"
