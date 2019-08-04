#!/bin/sh

# upload the overview
cd index
./deploy.sh
cd ..

# build and upload the contributers guide
cd contributers
./deploy.sh
cd ..

# build and upload the HTTP-API documentation
cd http-api
./deploy.sh
cd ..

# build and upload the rust documentation
cd ..
cargo doc
cp target/doc/settings.html target/doc/index.html
rsync -azhe "ssh -p 2222" ./target/doc/ admin@docs.qaul.net:/home/admin/api
cd docs
