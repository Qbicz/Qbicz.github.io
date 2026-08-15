#!/bin/bash
set -e

echo "Building Leptos app..."
trunk build --release

echo "Deploying to GitHub Pages..."
cd dist
cp index.html 404.html
git init
git add -A
git commit -m "Deploy"
git push -f git@github.com:Qbicz/Qbicz.github.io.git master:gh-pages

echo "Deployed successfully!"
