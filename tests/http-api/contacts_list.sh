#!/bin/bash

# returns a list of all contacts
#
# usage:
# ./contacts_list.sh

http 127.0.0.1:9900/rest/contacts \
  "Authorization:{\"id\":\"$QAUL_ID\",\"token\":\"$QAUL_TOKEN\"}"
