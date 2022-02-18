#!/usr/bin/env bash

# qaul documentation deployment script

# build and upload the rust API documentation
cd ../rust
cargo doc --no-deps
cp target/doc/settings.html target/doc/index.html
rsync -azzhe "ssh -p 2222" ./target/doc/ admin@docs.qaul.net:/home/admin/api
cd ../docs

