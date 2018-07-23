#!/bin/bash
rev=$(git rev-parse --short HEAD)

cd target/doc

git init
git config user.name "Nathan Lilienthal"
git config user.email "nathan@nixpulvis.com"

git remote add upstream "https://$GH_TOKEN@github.com/nixpulvis/oursh"
git fetch upstream && git reset upstream/gh-pages

touch .

git add -A .
git commit -m "rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages

