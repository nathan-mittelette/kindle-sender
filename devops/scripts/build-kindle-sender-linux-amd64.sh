#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository_dir="$(cd "${script_dir}/../.." && pwd)"
artifact_dir="${repository_dir}/dist/kindle-sender-linux-amd64-gnu"

mkdir -p "${artifact_dir}"

docker buildx build \
  --platform linux/amd64 \
  --file "${repository_dir}/devops/docker/Dockerfile.kindle-sender" \
  --target artifact \
  --output "type=local,dest=${artifact_dir}" \
  "${repository_dir}"

echo "Binary created at ${artifact_dir}/kindle-sender"
file "${artifact_dir}/kindle-sender"
