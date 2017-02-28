#!/bin/bash
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

    cargo clean
    cross rustc --path $CRATE_NAME --target $TARGET --release --features="$FEATURE"

    # copy linux
    cp target/$TARGET/release/lib$CRATE_NAME* $stage/
    cp $CRATE_NAME/README.md $stage/
    cp $CRATE_NAME/LICENSE $stage/
    cp $CRATE_NAME/CHANGELOG.md $stage/

    cd $stage

    if [ ! -z $TARGET_NAME ]; then
        zip $src/$CRATE_NAME$FEATURE_NAME-$TRAVIS_TAG-$TARGET_NAME.zip *
    else
        zip $src/$CRATE_NAME$FEATURE_NAME-$TRAVIS_TAG-$TARGET.zip *
    fi
    cd $src



    rm -rf $stage
}


declare -a CRATES=("safe_app" "safe_authenticator")
declare -a FEATURES=("use-mock-routing testing,dev")

for crate in "${CRATES[@]}"
do
    export CRATE_NAME="$crate"
    if [ $TRAVIS_OS_NAME = linux ]; then

        declare -a TARGETS=("i686-unknown-linux-gnu,linux-x32"
                            "x86_64-unknown-linux-gnu,linux-x64"
                            "i686-unknown-linux-musl"
                            "x86_64-unknown-linux-musl"
                            )
    else
        declare -a TARGETS=("x86_64-apple-darwin,darwin-x64"
                            "i686-apple-darwin,darwin-x32"
                            )
    fi

    for target in "${TARGETS[@]}"
    do
        export TARGET=${target%,*}       # before comma
        export TARGET_NAME=${target#*,}   # after comma

        for feat in "${FEATURES[@]}"
        do
            export FEATURE=${feat%,*}          # before comma
            export FEATURE_NAME="-${feat#*,}"   # after comma
            main
        done
    done

    # # move the package up
    # cp *.zip ../
    # # and leave the create
    # cd ..
done