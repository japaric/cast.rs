set -ex

main() {
    cross test --target $TARGET
    cross test --target $TARGET --release

    cross test --no-default-features --target $TARGET
    cross test --no-default-features --target $TARGET --release
}

main
