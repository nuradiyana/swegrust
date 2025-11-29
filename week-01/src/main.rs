use std::io;
use std::io::Write;

const APP_NAME: &str = "My App";

enum Sapaan {
    Formal,
    Santai,
    Inggris,
}

impl Sapaan {
    fn pilih(pilihan: &str) -> Result<Sapaan, String> {
        match pilihan {
            "1" => Ok(Sapaan::Formal),
            "2" => Ok(Sapaan::Santai),
            "3" => Ok(Sapaan::Inggris),
            _ => Err(String::from(
                "Pilihan tidak valid! Harap masukkan angka 1-3.",
            )),
        }
    }
}

struct Pengguna {
    nama: String,
    sapaan: Sapaan,
}

impl Pengguna {
    fn sapa(&self) {
        match self.sapaan {
            Sapaan::Formal => {
                println!("Selamat Pagi, Bapak/Ibu {}.", self.nama)
            }
            Sapaan::Santai => {
                println!("Halo {}, apa kabar?", self.nama)
            }
            Sapaan::Inggris => {
                println!("Hello {}, have a nice day!", self.nama);
            }
        }
    }

    fn baru(nama: String, sapaan: Sapaan) -> Pengguna {
        Pengguna { nama, sapaan }
    }
}

fn main() {
    println!("--- Memulai {} ---", APP_NAME);

    print!("Masukan nama anda: ");
    io::stdout().flush().unwrap();

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Gagal membaca input nama");

    let name = name.trim().to_string();

    println!("\nPilih Gaya Sapaan: ");
    println!("1. Formal");
    println!("2. Santai");
    println!("3. Inggris");
    print!("Pilihan Anda (1-3): ");
    io::stdout().flush().unwrap();

    let mut sapaan = String::new();
    io::stdin()
        .read_line(&mut sapaan)
        .expect("Gagal membaca input pilihan sapaan");

    let pilihan = Sapaan::pilih(sapaan.trim());
    let mode = match pilihan {
        Ok(mode) => mode,
        Err(err) => {
            println!("\nError: {}", err);
            std::process::exit(1);
        }
    };

    let pengguna = Pengguna::baru(name, mode);

    println!("\n--------------------------");
    pengguna.sapa();
    println!("--------------------------");
}
