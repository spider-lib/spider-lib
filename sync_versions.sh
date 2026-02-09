#!/bin/bash
# Script to synchronize versions across workspace crates

set -e  # Exit on any error

echo "Synchronizing local workspace dependency versions..."
echo "--- Collecting current versions from all workspace crates..."

# Clean up any previous temporary file
rm -f .versions.tmp

# Extract workspace members from Cargo.toml
MEMBERS=$(grep -A 20 "^\[workspace\]" Cargo.toml | grep -E "^\s*\"" | sed 's/^[[:space:]]*"\([^"]*\)".*/\1/' | grep -v '^$')

# Process each member
for dir in $MEMBERS; do
    # Remove quotes and trim whitespace
    dir=$(echo "$dir" | sed 's/"//g' | xargs)
    
    if [ -n "$dir" ] && [ -d "$dir" ] && [ -f "$dir/Cargo.toml" ]; then
        CRATE_NAME=$(grep -m 1 "^name =" "$dir/Cargo.toml" | cut -d '"' -f 2)
        CRATE_VERSION=$(grep -m 1 "^version =" "$dir/Cargo.toml" | cut -d '"' -f 2)
        
        if [ -n "$CRATE_NAME" ] && [ -n "$CRATE_VERSION" ]; then
            echo "$CRATE_NAME,$CRATE_VERSION" >> .versions.tmp
            echo "Found crate: $CRATE_NAME with version: $CRATE_VERSION"
        fi
    fi
done

# Check if we found any crates
if [ ! -f ".versions.tmp" ] || [ ! -s ".versions.tmp" ]; then
    echo "Warning: No crates found or .versions.tmp file is empty."
    echo "Dependency synchronization complete!"
    exit 0
fi

# Update versions in all Cargo.toml files
echo "--- Updating dependency versions in all Cargo.toml files..."
for target_cargo in $(find . -maxdepth 2 -name "Cargo.toml"); do
    echo "    Processing $target_cargo"
    
    # Process each dependency to update its version
    while IFS=, read -r DEP_NAME DEP_VERSION; do
        if [ -n "$DEP_NAME" ] && [ -n "$DEP_VERSION" ]; then
            # Update version in path-based dependencies: dep_name = { version = "x.x.x", path = "..." }
            sed -i.bak -E "s/^($DEP_NAME\s*=\s*\{[^\}]*version\s*=\s*\")[^\"]*(\"[^\}]*path.*)/\1$DEP_VERSION\2/g" "$target_cargo" 2>/dev/null || true
            
            # Update version in simple dependencies: dep_name = "x.x.x"
            sed -i.bak -E "s/^($DEP_NAME\s*=\s*\")[^\"]*(\")/\1$DEP_VERSION\2/g" "$target_cargo" 2>/dev/null || true
            
            # Clean up backup file
            rm -f "$target_cargo.bak" 2>/dev/null || true
        fi
    done < .versions.tmp
done

# Clean up temporary file
rm -f .versions.tmp

echo "Dependency synchronization complete!"