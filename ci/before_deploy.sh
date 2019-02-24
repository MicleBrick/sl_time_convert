# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    cross rustc --bin sl_time_convert --target x86_64-pc-windows-gnu --target TARGET=x86_64-apple-darwin --target TARGET=x86_64-unknown-linux-gnu --release -- -C lto

    # TODO Update this to package the right artifacts
    cp target/$TARGET/release/hello $stage/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
