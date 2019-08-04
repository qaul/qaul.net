#!/bin/sh

# create book
./build.sh

# upload book to web server
rsync -azhe "ssh -p 2222" ./book/ admin@docs.qaul.net:/home/admin/contributers
