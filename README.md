# Form Builder - RISTEK Web Development SIG Task

Proyek ini adalah implementasi dari tugas seleksi RISTEK Web Development SIG. Aplikasi ini mengadopsi arsitektur terpisah (Decoupled Architecture) dengan antarmuka modern dan backend API berkinerja tinggi yang aman secara memori.

## Tech Stack Utama
* **Frontend:** Next.js (App Router), Tailwind CSS
* **Backend:** Rust, Axum, Tokio
* **Database:** PostgreSQL (Hosted on Neon.tech), SQLx (Compile-time checked queries)
* **Authentication:** JWT (JSON Web Tokens), Bcrypt Hashing

## Struktur Proyek
Proyek ini menggunakan struktur Monorepo:
* `/frontend`: Berisi antarmuka pengguna (UI) Next.js.
* `/backend`: Berisi logika REST API dan koneksi database menggunakan Rust.

## 🛠️ Cara Menjalankan Proyek secara Lokal

### Prasyarat
Pastikan komputer Anda sudah terinstal:
* [Node.js](https://nodejs.org/) (untuk menjalankan Frontend)
* [Rust & Cargo](https://rustup.rs/) (untuk mengkompilasi Backend)

### 1. Setup Backend (Rust)
1. Buka terminal dan masuk ke direktori backend:
   ```bash
   cd backend
   ```
2. Buat file `.env` di dalam folder `backend` dan masukkan Connection String PostgreSQL serta Secret JWT Anda (Anda bisa menggunakan database lokal atau cloud seperti Neon):
   ```env
   DATABASE_URL="postgres://<username>:<password>@<host>/<dbname>?sslmode=require"
   JWT_SECRET="rahasia_super_aman"
   ```
3. Jalankan server backend (proses kompilasi pertama mungkin memakan waktu):
   ```bash
   cargo run
   ```
   *Server backend akan berjalan di `http://127.0.0.1:8000`.*

### 2. Setup Frontend (Next.js)
1. Buka terminal baru dan masuk ke direktori frontend:
   ```bash
   cd frontend
   ```
2. Install semua dependensi:
   ```bash
   npm install
   ```
3. Jalankan development server:
   ```bash
   npm run dev
   ```
   *Aplikasi frontend dapat diakses melalui browser di `http://localhost:3000`.*

## Fitur yang Diimplementasikan (Level 1)
* **Authentication:** Registrasi pengguna baru dan Login (Password di-hash menggunakan Bcrypt, sesi diamankan menggunakan JWT).
* **Proteksi Route:** Pengguna harus login untuk membuat Form (Middleware Auth Guard).
* **Form CRUD Dasar:** Pengguna terautentikasi dapat membuat Form baru dan melihat daftar Form.