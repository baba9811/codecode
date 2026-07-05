#!/usr/bin/env bash
set -euo pipefail

version="${1:-}"
if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "usage: scripts/release.sh 0.1.1" >&2
  exit 2
fi

VERSION="$version" perl -0pi -e 's/^version = ".*"/version = "$ENV{VERSION}"/m' Cargo.toml
node -e "const fs=require('fs'); const p=require('./package.json'); p.version=process.env.VERSION; fs.writeFileSync('package.json', JSON.stringify(p, null, 2) + '\n')"
cargo check

git add Cargo.toml Cargo.lock package.json
git commit -m "Release v$version"
git tag "v$version"
echo "push with: git push origin main v$version"
