#!/usr/bin/env bash
# publish.sh — build this fork's oxlint and publish it to npm as @stavalfi/oxc,
# consumed by the poc monorepo through an npm alias.
#
# This builds the napi addon and the JS bundle, exactly as upstream's npm package
# does, NOT the standalone `apps/oxlint` binary. The standalone binary cannot run
# `jsPlugins` — JS plugin rules need the Node runtime that hosts the addon, so
# under a bare Rust binary every custom-oxlint-rules/* rule in poc silently
# reports nothing. Verified: upstream npm oxlint 1.86.0 reports violations from
# poc's JS plugin where the standalone 1.86 binary reports none.
#
# The .node addon is shipped inside the package rather than as a per-platform
# optionalDependency, so a single package covers every platform. The
# generated bindings.js already prefers a local ./oxlint.<platform>.node over the
# @oxlint/binding-<platform> package, which is what makes that work.
set -euo pipefail

# cmake builds mimalloc, which the napi build pulls in via --features allocator.
if ! command -v cargo >/dev/null 2>&1 || ! command -v cmake >/dev/null 2>&1; then
  exec nix shell nixpkgs#rustup nixpkgs#cargo-zigbuild nixpkgs#zig nixpkgs#cmake \
    --command "${BASH_SOURCE[0]}" "$@"
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$repo_root"

version="$(git describe --tags --abbrev=0 --match 'oxlint_v*' | sed 's/^oxlint_v//')-$(git rev-parse HEAD)"
tarball="stavalfi-oxc-${version}.tgz"

rustup show active-toolchain >/dev/null

if [[ ! -d node_modules ]]; then
  echo ">> pnpm install"
  pnpm install --frozen-lockfile
fi

# `build` == build-napi-release + build-js. The release profile it uses is the
# workspace [profile.release]: opt-level=3, lto=fat, codegen-units=1,
# panic=abort, strip=symbols. --features allocator adds mimalloc.
echo ">> building the napi addon and JS bundle"
(cd apps/oxlint && pnpm run build)

stage="$(mktemp -d)/package"
trap 'rm -rf "$(dirname "$stage")"' EXIT
cp -r oasis/npm-package "$stage"
mkdir -p "$stage/bin"
cp -r apps/oxlint/dist "$stage/dist"
cp LICENSE README.md npm/oxlint/configuration_schema.json "$stage/"
cp npm/oxlint/bin/oxlint "$stage/bin/oxlint"

# bindings.js resolves ./oxlint.<platform>.node relative to its own directory,
# which is dist/ once bundled.
cp apps/oxlint/src-js/*.node "$stage/dist/"

npm --prefix "$stage" version "$version" --no-git-tag-version --allow-same-version >/dev/null

out="$(dirname "$stage")/${tarball}"
tar -czf "$out" -C "$(dirname "$stage")" package
# Staged, not published: staging needs no 2FA, and the version only goes public
# once a maintainer approves it with 2FA. `npm stage` needs npm >= 11.15, newer
# than the npm on PATH, hence the pinned npx. Every version is a `<tag>-<sha>`
# prerelease, which npm refuses without an explicit tag.
npx -y npm@12.2.1 stage publish "$out" --access public --tag latest

echo
echo "approve it (2FA): npx -y npm@12.2.1 stage list @stavalfi/oxc  then  npx -y npm@12.2.1 stage approve <stage-id>"
echo "  or on npmjs.com -> @stavalfi/oxc -> Staged Packages -> Approve"
echo "poc package.json -> \"oxlint\": \"npm:@stavalfi/oxc@${version}\""
