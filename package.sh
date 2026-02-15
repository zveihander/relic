#!/bin/sh

set -e

VERSION=$(grep "^version =" Cargo.toml | cut -d '"' -f 2)
OUT_DIR="dist"

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

TARGETS="aarch64-unknown-linux-gnu aarch64-unknown-linux-musl x86_64-unknown-linux-gnu x86_64-unknown-linux-musl"

for TARGET in $TARGETS; do
    echo "--- Building for $TARGET ---"

    RUSTFLAGS="-Zlocation-detail=none -Zfmt-debug=none -Zunstable-options -Cpanic=immediate-abort" cargo +nightly zigbuild \
        -Z build-std=std,panic_abort -Z build-std-features="optimize_for_size" \
        --release --target "$TARGET"

    BIN_PATH="target/$TARGET/release/relic"

    TAR_NAME="relic-$VERSION-$TARGET.tar.gz"
    tar -czf "$OUT_DIR/$TAR_NAME" -C "target/$TARGET/release" relic

    echo "Created: $OUT_DIR/$TAR_NAME"
done

echo "--- Generating Checksums ---"
cd "$OUT_DIR"
sha256sum *.tar.gz >SHA256SUMS
cd ..

echo "--- Packaging Finished ---"
ls -lh "$OUT_DIR"
