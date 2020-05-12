#!/bin/sh

# logout from session
# 
# usage
# ./logout.sh
#

http GET 127.0.0.1:9900/rest/logout \
    "Authorization:{\"id\":\"$QAUL_ID\",\"token\":\"$QAUL_TOKEN\"}"
