# Week 01 — Program Sapaan CLI (Rust)

Program CLI sederhana untuk menyapa pengguna dengan beberapa gaya sapaan (Formal, Santai, Inggris). Proyek ini ditulis menggunakan Rust dan dirancang untuk latihan dasar: membaca input, menggunakan enum dan struct, serta penanganan error sederhana.

## Isi Proyek

- `src/main.rs` — Implementasi program utama
- `Cargo.toml` — Konfigurasi crate (Edition 2024)

## Prasyarat

- Rust toolchain (stable) yang mendukung Edition 2024
  - Instal via: https://rustup.rs
  - Verifikasi: `rustc --version` dan `cargo --version`

## Menjalankan Aplikasi

1. Masuk ke direktori proyek:
   ```bash
   cd week-01
   ```
2. Jalankan aplikasi:
   ```bash
   cargo run
   ```

## Contoh Interaksi

```
--- Memulai My App ---
Masukan nama anda: John

Pilih Gaya Sapaan: 
1. Formal
2. Santai
3. Inggris
Pilihan Anda (1-3): 2

--------------------------
Halo John, apa kabar?
--------------------------
```

Jika Anda memasukkan pilihan yang tidak valid (selain 1–3), program akan menampilkan pesan error dan keluar.

## Arsitektur Singkat

- `enum Sapaan { Formal, Santai, Inggris }`
  - Dipilih berdasarkan input angka 1–3.
- `struct Pengguna { nama: String, sapaan: Sapaan }`
  - Memiliki method `sapa()` untuk mencetak salam sesuai gaya sapaan.
- Menggunakan `stdout().flush()` agar prompt tampil sebelum input dibaca.

## Lisensi

Bebas digunakan untuk keperluan belajar.
