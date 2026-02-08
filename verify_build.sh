#!/bin/bash

# Script untuk memverifikasi build semua crate sebelum publish
# Menjalankan cargo check di setiap crate untuk memastikan tidak ada error

set -e  # Exit jika ada error

echo "Memverifikasi spider-util..."
cd spider-util
cargo check
cd ..

echo "Memverifikasi spider-macro..."
cd spider-macro
cargo check
cd ..

echo "Memverifikasi spider-downloader..."
cd spider-downloader
cargo check
cd ..

echo "Memverifikasi spider-middleware..."
cd spider-middleware
cargo check
cd ..

echo "Memverifikasi spider-pipeline..."
cd spider-pipeline
cargo check
cd ..

echo "Memverifikasi spider-core..."
cd spider-core
cargo check
cd ..

echo "Verifikasi selesai! Semua crate siap untuk di-publish."