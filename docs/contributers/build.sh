#!/bin/sh

dot -Tsvg src/assets/dependencies.dot -o src/assets/dependencies.svg
mdbook build