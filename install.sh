#!/bin/bash

cargo build --release
sudo cp target/release/ax /usr/local/bin/ax
