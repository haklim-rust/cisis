# cisis (Alpha v0.1.0)

**A High-Performance File Identification Tool for Resource-Constrained Environments.**

cisis adalah utilitas sistem berbasis Rust yang dirancang untuk menggantikan fungsionalitas `file` tradisional dengan fokus pada kecepatan eksekusi dan akurasi deteksi pada sistem Termux/Linux.

## Core Capabilities

cisis beroperasi menggunakan metodologi deteksi berlapis untuk memastikan identifikasi aset digital yang presisi:

* **Signature-Based Analysis:** Menggunakan basis data *magic numbers* untuk identifikasi instan pada file biner, image, dan media.
* **Deep ISO 9660 Inspection:** Protokol khusus untuk mendeteksi struktur *disk image* yang tidak terbaca oleh parser standar.
* **Shebang & Script Parsing:** Analisis baris pertama (Interpreter Directive) untuk validasi skrip Shell, Python, dan Node.js.
* **Contextual Filename Matching:** Identifikasi cerdas untuk file sistem tanpa ekstensi seperti `.gitignore`, `.env`, dan `Makefile`.
* **Parallel Processing Engine:** Memanfaatkan model *concurrency* Rust untuk memproses ribuan entri file secara simultan.



## Technical Specifications

| Feature | Specification |
|:---|:---|
| **Core Engine** | Rust 2021 Edition |
| **Concurrency** | Rayon-based Work-Stealing |
| **Memory Safety** | Zero-copy Buffer Analysis |
| **Dependency** | Minimalist Static Linking |
| **Platform** | AArch64 (Termux), x86_64 (Linux) |

## Deployment

Proyek ini saat ini berada dalam fase **Alpha**. Build binary tersedia melalui sistem manajemen paket Cargo:

```bash
cargo install --git [https://github.com/username/cisis](https://github.com/username/cisis) --tag v0.1.0-alpha.1
