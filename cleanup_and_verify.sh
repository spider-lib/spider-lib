#!/bin/bash

# Script untuk membersihkan dan mengupdate workspace sebelum publish
# Menjalankan cargo clean dan cargo update di workspace utama

set -e  # Exit jika ada error

echo "Membersihkan workspace..."
cargo clean

echo "Mengupdate dependensi..."
cargo update

echo "Menjalankan build untuk memastikan semuanya berjalan..."
cargo build

echo "Build berhasil! Workspace siap untuk publish."