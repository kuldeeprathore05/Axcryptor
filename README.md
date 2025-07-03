# 🔐 Axcryptor – Secure File Encryption Web App

[![Shuttle Deploy](https://img.shields.io/badge/deployed%20on-shuttle-blue.svg)](https://axcryptor-y3ua.shuttle.app/)

Axcryptor is a secure, browser-based file encryption tool built using **Rust** and the **Axum** web framework. It allows users to encrypt and decrypt files using **AES-256** or **ChaCha20**, with support for **batch processing** and **streaming large files** — all processed **client-side** with no server-side file storage.

---

## Security Highlights

- ✅ Built in **Rust** with strong type safety
- ✅ Uses [aes](https://docs.rs/aes), [chacha20poly1305](https://docs.rs/chacha20poly1305), and [argon2](https://docs.rs/argon2) crates
- ✅ No disk writes — files are never stored on the server
- ✅ Password-based encryption only (no key reuse or hardcoded secrets)

---

## 🧰 Built With

- [Rust](https://www.rust-lang.org/)
- [Axum](https://docs.rs/axum)
- [Shuttle](https://www.shuttle.rs)
- HTML + CSS + JavaScript

--- 

## Features

- 🔐 Encrypt/Decrypt with AES-256 and ChaCha20
- 📂 Drag & Drop file upload
- 📦 Batch file encryption and decryption
- ☁️ Deployed using [Shuttle](https://axcryptor-y3ua.shuttle.app/)

---

## How It Works

The entire encryption/decryption process is visualized in this flowchart:

![Encryption-Decryption Flowchart](static/screens/flowchart.png)

---

## 🖥️ Run Locally

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.

```bash
git clone https://github.com/yourusername/axcryptor.git
cd axcryptor
cargo run
```
Now open http://localhost:3000 in your browser.

