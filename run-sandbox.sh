#!/usr/bin/env bash

set -euo pipefail

#make sure we are in the actual directory of this script.
#We cd into the dirname path and use pwd and copare it with the actual pdf
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CURRENT_DIR="$(pwd)"
if [ "$CURRENT_DIR" != "$SCRIPT_DIR" ]; then
	echo "This script MUST be run from its absolute direktory as pwd"
	exit 1
fi

#Check for dockerfile
if [ ! -f "Dockerfile" ]; then
	echo "No dockerfile in directory. Quit"
	exit 1
fi

docker run -it --rm \
	-v "$CURRENT_DIR":/workdir \
	--cap-drop=ALL \
	--security-opt=no-new-privileges \
	--name go-sandbox \
	$(docker build -q -f Dockerfile .)
