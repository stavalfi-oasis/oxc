#!/usr/bin/env bash
# publish.sh — build this fork's standalone `oxlint` binary (apps/oxlint, not the
# napi addon npm ships) and publish the npm/oxlint-poc package as a tarball to the
# Oasis rustfs bucket, consumed by the poc monorepo as a tarball-URL dependency.
set -euo pipefail

# Provision the Rust toolchain from nixpkgs when it isn't already on PATH.
# zig is the cross-linker cargo-zigbuild drives for the linux target.
if ! command -v cargo >/dev/null 2>&1; then
  exec nix shell nixpkgs#rustup nixpkgs#cargo-zigbuild nixpkgs#zig \
    --command "${BASH_SOURCE[0]}" "$@"
fi

S3_ENDPOINT="${S3_ENDPOINT:-http://127.0.0.1:9100}"
S3_BUCKET="${S3_BUCKET:-oasis-npm}"
export AWS_ACCESS_KEY_ID="${AWS_ACCESS_KEY_ID:-minioadmin}"
export AWS_SECRET_ACCESS_KEY="${AWS_SECRET_ACCESS_KEY:-minioadmin}"
export AWS_REGION="${AWS_REGION:-us-east-1}"
unset AWS_PROFILE

# Space-separated npm platform names. Override to shorten the edit loop, e.g.
# TARGETS=darwin-arm64 ./publish.sh
TARGETS="${TARGETS:-darwin-arm64 linux-x64}"

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$repo_root"

version="$(git describe --tags --abbrev=0 --match 'oxlint_v*' | sed 's/^oxlint_v//')-poc-$(git rev-parse HEAD)"
tarball="oxlint-${version}.tgz"

stage="$(mktemp -d)/package"
trap 'rm -rf "$(dirname "$stage")"' EXIT
cp -r npm/oxlint-poc "$stage"
cp LICENSE README.md npm/oxlint/configuration_schema.json "$stage/"
npm --prefix "$stage" version "$version" --no-git-tag-version --allow-same-version >/dev/null

rustup show active-toolchain >/dev/null

for npm_platform in $TARGETS; do
  case "$npm_platform" in
    # target-cpu is a portability floor, not `native`: the tarball has to run on
    # every teammate's machine, not just the one that built it.
    darwin-arm64) triple=aarch64-apple-darwin;      builder=build;    cpu=apple-m1 ;;
    linux-x64)    triple=x86_64-unknown-linux-gnu;  builder=zigbuild; cpu=x86-64-v3 ;;
    *) echo "unknown target: $npm_platform" >&2; exit 1 ;;
  esac

  echo ">> building oxlint for ${npm_platform} (${triple})"
  rustup target add "$triple"
  # The workspace [profile.release] already pins opt-level=3, lto=fat,
  # codegen-units=1, panic=abort — only target-cpu is left to set here.
  RUSTFLAGS="-C target-cpu=${cpu}" \
    cargo "$builder" --release --target "$triple" -p oxlint --bin oxlint
  cp "target/${triple}/release/oxlint" "$stage/bin/oxlint-${npm_platform}"
done

out="$(dirname "$stage")/${tarball}"
tar -czf "$out" -C "$(dirname "$stage")" package
aws --endpoint-url "$S3_ENDPOINT" s3 cp "$out" "s3://${S3_BUCKET}/${tarball}" --no-progress

echo
echo "poc package.json -> \"oxlint\": \"${S3_ENDPOINT}/${S3_BUCKET}/${tarball}\""
