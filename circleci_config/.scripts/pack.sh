#!/usr/bin/env bash

# CircleCI CLI must be installed
if ! command -v circleci &> /dev/null
then
    echo "circleci could not be found"
    exit 1
fi

# Validate the generated configuration prior to replacing the current one
echo "Validating generated config.yml file..."
circleci config pack circleci_config | circleci config validate -

# Pack the configuration
echo ""
echo "Replacing current .circleci/config.yml file..."
{
  echo "# ---------------------------------------------------"
  echo "# ----- config.yml ----------------------------------"
  echo "# ----- GENERATED CODE - DO NOT MODIFY BY HAND ------"
  echo "# ---------------------------------------------------"
  echo ""
  circleci config pack circleci_config
} > .circleci/config.yml
echo "Done!"
