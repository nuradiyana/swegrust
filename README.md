# SWE Growth — Belajar Rust (swegrust)

Repositori ini berisi kumpulan proyek mingguan belajar Rust bersama SWE Growth. Setiap minggu memiliki folder sendiri (mis. `week-01`, `week-02`, dst.) yang berisi kode, README, dan materi pendukung.

## Struktur Repository

- `week-01/` — Program Sapaan CLI (dasar input/output, enum, struct).
- `week-02/` — (akan ditambahkan).
- ...

Setiap folder minggu memiliki README sendiri yang menjelaskan tujuan dan cara menjalankan contoh minggu tersebut.

## Prasyarat

- Rust toolchain (stable) — instal via https://rustup.rs

## Cara Menjalankan Proyek Mingguan

Misal menjalankan minggu pertama:
```bash
cd week-01
cargo run
```

Untuk minggu lainnya, masuk ke folder minggu terkait dan jalankan perintah yang sama.

## Konvensi & Standar

- Edition: 2024 (lihat `Cargo.toml` masing-masing minggu).
- Format kode: `cargo fmt`
- Struktur per-minggu: proyek binari sederhana (`cargo new --bin week-XX`).

## Sumber Belajar yang Disarankan

- The Rust Book: https://doc.rust-lang.org/book/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- Rustlings: https://github.com/rust-lang/rustlings
- Edition Guide (2024): https://doc.rust-lang.org/edition-guide/

## Lisensi

Konten dalam repo ini ditujukan untuk keperluan belajar bersama. Gunakan dan modifikasi seperlunya.
