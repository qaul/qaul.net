#!/bin/sh

# build qaul rust documentation
cargo doc

# create index file
cp ../target/doc/settings.html ../target/doc/index.html

# upload documentation to api.qaul.net
rsync -azhe "ssh -p 2224" ../target/doc/ admin@qaul.net:/home/admin
