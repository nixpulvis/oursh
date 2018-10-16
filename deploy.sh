#!/bin/bash

# Get the current git revision short name.
rev=$(git rev-parse --short HEAD)

# Assume the docs are already built...
cd target/doc

# TODO: https://github.com/rust-lang/rust/issues/16103#issuecomment-424054063
ls -s oursh/index.html index.html

# Create a new clone of the git repository here.
git init
# TODO: Deploy as a deployer, or others?
git config user.name "Nathan Lilienthal"
git config user.email "nathan@nixpulvis.com"
# Create a remote to the GitHub repository.
git remote add upstream "https://$GH_TOKEN@github.com/nixpulvis/oursh"
# Fetch, and checkout to the GitHub Pages branch.
git fetch upstream && git reset upstream/gh-pages

touch .
git add -A .

# Commit the new build.
git commit -m "rebuild pages at ${rev}"
# Push the new build.
git push -q upstream HEAD:gh-pages
