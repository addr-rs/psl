#!/bin/sh

set -e

cargo make test

if ! git diff --exit-code --quiet data/rules.txt; then
	git commit -am 'update the list'
	cargo release --no-confirm
fi
