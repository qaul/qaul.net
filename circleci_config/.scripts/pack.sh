#!/usr/bin/env bash

# CircleCI CLI must be installed
circleci version

# Validate the generated configuration prior to replacing the current one
circleci config pack circleci_config | circleci config validate -

# Pack the configuration
touch /tmp/.tmpymlfile
{
  echo "# ---------------------------------------------------"
  echo "# ----- config.yml ----------------------------------"
  echo "# ----- GENERATED CODE - DO NOT MODIFY BY HAND ------"
  echo "# ---------------------------------------------------"
  echo ""
  circleci config pack circleci_config
} >.circleci/config.yml

