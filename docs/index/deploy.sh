#!/bin/sh

# upload index file & favicon
scp -P 2222 index.html admin@docs.qaul.net:/home/admin/
scp -P 2222 favicon.ico admin@docs.qaul.net:/home/admin/
