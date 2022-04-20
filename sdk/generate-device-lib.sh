#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd $SCRIPT_DIR;

[-e nrf51822] && rm -rf nrf51822
cargo new --lib nrf51822

cd nrf51822

# from https://developer.nordicsemi.com/nRF51_SDK/nRF51_SDK_v10.x.x/
svd2rust -i "$SCRIPT_DIR/nrf51822_sdk/SVD/nrf51.svd"
form -i lib.rs -o src/ && rm lib.rs

cargo fmt