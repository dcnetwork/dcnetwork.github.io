#!/bin/bash

echo "Pushing to GitHub ... [DC Network]"

git add .

git commit -m "$1"

if [ -d docs ]; then
	rm -r -f docs
fi

if [ -d dist ]; then 
	mv dist docs
fi

git push -u origin main
