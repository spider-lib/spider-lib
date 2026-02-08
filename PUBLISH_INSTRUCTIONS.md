# Langkah-langkah Publish ke Crates.io

Ikuti langkah-langkah berikut untuk mempublish semua submodule ke crates.io:

## 1. Persiapan
Pastikan Anda memiliki:
- Akun crates.io
- API token dari crates.io
- Hak akses untuk mempublish semua crate yang terlibat

## 2. Login ke Cargo
```bash
cargo login YOUR_API_TOKEN_HERE
```

## 3. Verifikasi Build
Jalankan skrip verifikasi untuk memastikan semua crate dapat dibuild tanpa error:
```bash
chmod +x verify_build.sh
./verify_build.sh
```

## 4. Cleanup dan Update Dependencies
Opsional: Jalankan skrip cleanup untuk memperbarui workspace:
```bash
chmod +x cleanup_and_verify.sh
./cleanup_and_verify.sh
```

## 5. Publish Semua Crate
Jalankan skrip publish untuk mempublish semua crate secara berurutan:
```bash
chmod +x publish_crates.sh
./publish_crates.sh
```

## 6. Verifikasi di Crates.io
Setelah selesai, pastikan semua crate telah diperbarui di https://crates.io/

## Catatan Penting
- Pastikan untuk mempublish crate dalam urutan yang benar (dependensi harus dipublish terlebih dahulu)
- Urutan publish yang direkomendasikan:
  1. spider-util
  2. spider-macro
  3. spider-downloader
  4. spider-middleware
  5. spider-pipeline
  6. spider-core
- Jika ada perubahan yang mempengaruhi dependensi antar crate, Anda mungkin perlu mengupdate dan mempublish ulang crate yang bergantung padanya