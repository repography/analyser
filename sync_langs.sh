#!/bin/bash
curl https://raw.githubusercontent.com/github/linguist/master/lib/linguist/languages.yml \
	| yq eval -o=j \
	| jq 'keys[] as $k | select(.[$k].extensions) | {($k): .[$k].extensions}' \
	| jq -s '. | add' \
	| ./gen_language_index.py \
	> src/languages.rs
