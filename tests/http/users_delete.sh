#!/bin/bash

# delete the currently authorized user
#
# usage:
# ./users_delete.sh

http DELETE 127.0.0.1:9900/rest/users/$QAUL_ID \
  purge:=true \
  "Authorization:{\"id\":\"$QAUL_ID\",\"token\":\"$QAUL_TOKEN\"}"
