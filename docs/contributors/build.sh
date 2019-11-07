#!/bin/sh

# build this mdbook
dot -Tsvg src/assets/dependencies.dot -o src/assets/dependencies.svg
mdbook build
