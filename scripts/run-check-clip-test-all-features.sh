# Find the directory this script is in
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Iterate until the root to locate the directory where Cargo.toml is and the first bytes seen are [workspace]
ROOT_DIR="$SCRIPT_DIR"
while [ "$ROOT_DIR" != "/" ]; do
    if [ -f "$ROOT_DIR/Cargo.toml" ] && grep -q '^\[workspace\]' "$ROOT_DIR/Cargo.toml"; then
        break
    fi
    ROOT_DIR="$(dirname "$ROOT_DIR")"
done

# Check if the ROOT_DIR is found
if [ "$ROOT_DIR" == "/" ]; then
    echo "Error: Could not find the directory containing Cargo.toml with [workspace]"
    exit 1
fi

echo "ROOT_DIR is set to $ROOT_DIR"

# Give read permissions to all the necessary scripts
chmod +rx "$ROOT_DIR/scripts/check-all-features.sh"
chmod +rx "$ROOT_DIR/scripts/clippy-all-features.sh"
chmod +rx "$ROOT_DIR/scripts/package-all-features.sh"
chmod +rx "$ROOT_DIR/scripts/tests-all-features.sh"

# Run check-all-features.sh
echo "Running check-all-features.sh"
"$ROOT_DIR/scripts/check-all-features.sh"

# Run clippy-all-features.sh
echo "Running clippy-all-features.sh"
"$ROOT_DIR/scripts/clippy-all-features.sh"

# Run package-all-features.sh
echo "Running package-all-features.sh"
"$ROOT_DIR/scripts/package-all-features.sh"

# Run tests-all-features.sh
echo "Running tests-all-features.sh"
"$ROOT_DIR/scripts/tests-all-features.sh"
