#!/bin/bash
# Release script for Dampen
# Usage: ./scripts/release.sh <version>
# Example: ./scripts/release.sh 0.2.0

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if version argument is provided
if [ -z "$1" ]; then
    echo -e "${RED}Error: Version argument required${NC}"
    echo "Usage: ./scripts/release.sh <version>"
    echo "Example: ./scripts/release.sh 0.2.0"
    exit 1
fi

NEW_VERSION=$1

# Validate version format (semver)
if ! [[ $NEW_VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo -e "${RED}Error: Invalid version format. Must be semver (e.g., 0.2.0)${NC}"
    exit 1
fi

echo -e "${GREEN}🚀 Starting release process for version $NEW_VERSION${NC}"
echo ""

# Check if working directory is clean
if [[ -n $(git status -s) ]]; then
    echo -e "${RED}Error: Working directory is not clean. Commit or stash changes first.${NC}"
    git status -s
    exit 1
fi

# Check if on main/master branch
CURRENT_BRANCH=$(git branch --show-current)
if [[ "$CURRENT_BRANCH" != "main" && "$CURRENT_BRANCH" != "master" ]]; then
    echo -e "${YELLOW}Warning: Not on main/master branch (current: $CURRENT_BRANCH)${NC}"
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

echo -e "${GREEN}Step 1: Running tests...${NC}"
# Note: We don't use --all-features because 'codegen' and 'interpreted' 
# are mutually exclusive in examples and would trigger a compile_error!
cargo test --workspace
echo ""

echo -e "${GREEN}Step 2: Running clippy...${NC}"
cargo clippy --workspace -- -D warnings
echo ""

echo -e "${GREEN}Step 3: Checking formatting...${NC}"
cargo fmt --all -- --check
echo ""

echo -e "${GREEN}Step 4: Updating version in Cargo.toml...${NC}"
# Update workspace package version
sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml

# Update dampen crate versions in workspace.dependencies
sed -i.bak "s/dampen-core = { path = \"\.\/crates\/dampen-core\", version = \".*\" }/dampen-core = { path = \".\/crates\/dampen-core\", version = \"$NEW_VERSION\" }/" Cargo.toml
sed -i.bak "s/dampen-macros = { path = \"\.\/crates\/dampen-macros\", version = \".*\" }/dampen-macros = { path = \".\/crates\/dampen-macros\", version = \"$NEW_VERSION\" }/" Cargo.toml
sed -i.bak "s/dampen-iced = { path = \"\.\/crates\/dampen-iced\", version = \".*\" }/dampen-iced = { path = \".\/crates\/dampen-iced\", version = \"$NEW_VERSION\" }/" Cargo.toml
sed -i.bak "s/dampen-cli = { path = \"\.\/crates\/dampen-cli\", version = \".*\" }/dampen-cli = { path = \".\/crates\/dampen-cli\", version = \"$NEW_VERSION\" }/" Cargo.toml
sed -i.bak "s/dampen-dev = { path = \"\.\/crates\/dampen-dev\", version = \".*\" }/dampen-dev = { path = \".\/crates\/dampen-dev\", version = \"$NEW_VERSION\" }/" Cargo.toml

rm -f Cargo.toml.bak
echo "  ✓ Updated workspace package version to $NEW_VERSION"
echo "  ✓ Updated dampen-core version to $NEW_VERSION"
echo "  ✓ Updated dampen-macros version to $NEW_VERSION"
echo "  ✓ Updated dampen-iced version to $NEW_VERSION"
echo "  ✓ Updated dampen-cli version to $NEW_VERSION"
echo "  ✓ Updated dampen-dev version to $NEW_VERSION"
echo ""

echo -e "${GREEN}Step 5: Building documentation...${NC}"
cargo doc --workspace --no-deps
echo ""

echo -e "${GREEN}Step 6: Creating git commit and tag...${NC}"
git add Cargo.toml
git commit -m "chore: bump version to $NEW_VERSION"
git tag -a "v$NEW_VERSION" -m "Release v$NEW_VERSION"
echo "  ✓ Created commit and tag v$NEW_VERSION"
echo ""

echo -e "${GREEN}✅ Release preparation complete!${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "1. Review the changes:"
echo "   git show"
echo ""
echo "2. Push the changes and tag:"
echo "   git push origin $CURRENT_BRANCH"
echo "   git push origin v$NEW_VERSION"
echo ""
echo "3. Create a GitHub Release at:"
echo "   https://github.com/mattdef/dampen/releases/new?tag=v$NEW_VERSION"
echo ""
echo "4. The GitHub Action will automatically publish to crates.io"
echo ""
echo -e "${YELLOW}To undo (if you made a mistake):${NC}"
echo "   git tag -d v$NEW_VERSION"
echo "   git reset --hard HEAD~1"
