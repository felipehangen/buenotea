#!/bin/bash
# Build script for Lambda deployment

set -e

echo "Building Lambda functions..."

# Build for AWS Lambda (x86_64-unknown-linux-musl)
cargo build --release --target x86_64-unknown-linux-musl --bin timing-batch
cargo build --release --target x86_64-unknown-linux-musl --bin sentiment-batch
cargo build --release --target x86_64-unknown-linux-musl --bin regime-batch
cargo build --release --target x86_64-unknown-linux-musl --bin fundamentals-batch
cargo build --release --target x86_64-unknown-linux-musl --bin invite-list-batch

echo "Creating deployment packages..."

mkdir -p ../../target/lambda

for func in timing-batch sentiment-batch regime-batch fundamentals-batch invite-list-batch; do
    cp ../../target/x86_64-unknown-linux-musl/release/$func ../../target/lambda/bootstrap
    cd ../../target/lambda
    zip ${func}.zip bootstrap
    cd -
    echo "Created ${func}.zip"
done

echo "Lambda functions built successfully!"
echo "Deployment packages are in target/lambda/"

