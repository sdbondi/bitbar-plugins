#!/usr/bin/env bash

# Here we set up our path and other vars for the rust executable

cd -P -- "$(dirname -- "$0")"
export NAMESPACES="production staging"
EXTRA_PATHS=

export PATH="/usr/local/bin:/usr/bin:$EXTRA_PATHS:$PATH"

./bins/k8spods
