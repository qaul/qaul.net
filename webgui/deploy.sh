#!/bin/sh

# Deploy the testing GUI to 
# https://ember-gui.qaul.net

# Build ember test
ember build

# Deploy to server
rsync -azhe "ssh -p 2422" ./dist/ admin@ember-gui.qaul.net:/home/admin/

