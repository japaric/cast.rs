set -ex

main() {
    cross test --target $TARGET
    cross test --target $TARGET --release

    [ -z "$RUST_NIGHTLY" ] || cross test --feature x128 --target $TARGET
    [ -z "$RUST_NIGHTLY" ] || cross test --feature x128 --target $TARGET --release

    cross test --no-default-features --target $TARGET
    cross test --no-default-features --target $TARGET --release
}

main
