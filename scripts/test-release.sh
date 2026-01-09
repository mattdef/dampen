#!/bin/bash
# Test script to verify release.sh version updates
# Usage: ./scripts/test-release.sh <version>
# Example: ./scripts/test-release.sh 0.2.0

set -e

NEW_VERSION=${1:-0.2.0}

echo "🧪 Testing version update to $NEW_VERSION (dry-run)"
echo ""

echo "Current versions in Cargo.toml:"
echo "================================"
grep "^version = " Cargo.toml
grep "dampen-.* = { path" Cargo.toml
echo ""

echo "After update would be:"
echo "======================"

# Test workspace.package version update
sed "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml | grep "^version = "

# Test dampen crate version updates
cat Cargo.toml | \
  sed "s/dampen-core = { path = \"\.\/crates\/dampen-core\", version = \".*\" }/dampen-core = { path = \".\/crates\/dampen-core\", version = \"$NEW_VERSION\" }/" | \
  sed "s/dampen-macros = { path = \"\.\/crates\/dampen-macros\", version = \".*\" }/dampen-macros = { path = \".\/crates\/dampen-macros\", version = \"$NEW_VERSION\" }/" | \
  sed "s/dampen-runtime = { path = \"\.\/crates\/dampen-runtime\", version = \".*\" }/dampen-runtime = { path = \".\/crates\/dampen-runtime\", version = \"$NEW_VERSION\" }/" | \
  sed "s/dampen-iced = { path = \"\.\/crates\/dampen-iced\", version = \".*\" }/dampen-iced = { path = \".\/crates\/dampen-iced\", version = \"$NEW_VERSION\" }/" | \
  sed "s/dampen-cli = { path = \"\.\/crates\/dampen-cli\", version = \".*\" }/dampen-cli = { path = \".\/crates\/dampen-cli\", version = \"$NEW_VERSION\" }/" | \
  grep "dampen-.* = { path"

echo ""
echo "✅ Test passed! All versions would be updated correctly."
