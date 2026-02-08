#!/bin/bash

# Script untuk publish semua crate spider-lib ke crates.io
# Pastikan Anda sudah login ke cargo terlebih dahulu dengan:
# cargo login <YOUR_API_TOKEN>

set -e  # Exit jika ada error

echo "Publishing spider-util..."
cd spider-util
cargo publish
cd ..

echo "Publishing spider-macro..."
cd spider-macro
cargo publish
cd ..

echo "Publishing spider-downloader..."
cd spider-downloader
cargo publish
cd ..

echo "Publishing spider-middleware..."
cd spider-middleware
cargo publish
cd ..

echo "Publishing spider-pipeline..."
cd spider-pipeline
cargo publish
cd ..

echo "Publishing spider-core..."
cd spider-core
cargo publish
cd ..

echo "Semua crate telah dipublish!"