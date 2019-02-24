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

    cross rustc --bin hello --target x86_64-pc-windows-gnu --release -- -C lto
    cross rustc --bin hello --target x86_64-apple-darwin --release -- -C lto
    cross rustc --bin hello --target x86_64-unknown-linux-gnu --release -- -C lto

    # TODO Update this to package the right artifacts
    cp target/x86_64-pc-windows-gnu/release/sl_time_convert $stage/windows
    cp target/x86_64-apple-darwin/release/sl_time_convert $stage/mac
    cp target/x86_64-unknown-linux-gnu/release/sl_time_convert $stage/linux

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-LINUX.tar.gz *
    cd $src

    rm -rf $stage
}

main
