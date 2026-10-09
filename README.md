# CAStudio — Autonomous Content Engineering Studio

[![Release](https://img.shields.io/badge/release-v0.1.0-8b5cf6.svg)](https://github.com/cecep-azhar/castudio)
[![License](https://img.shields.io/badge/license-Proprietary-blue.svg)]()
[![Platform](https://img.shields.io/badge/platform-Linux%20|%20Windows%20|%20macOS-green.svg)]()
[![CADS](https://img.shields.io/badge/CADS-v1.0%20Compliant-purple.svg)]()

> Sovereign, Local-First Content & Omnichannel Repurposing Studio untuk kreator, penulis buku, dan founder. Ubah satu ide/dokumen inti (Project Bible) menjadi multi-channel deliverables (E-Book, Instagram Carousel/Reels, Pitchdeck & SPK, Launch Pack) dengan otomatisasi penjadwalan.

---

## 🌟 Modul & Fitur Utama

- **Project & Project Bible Core**:
  - Sentralisasi ide, target audiens, dan *voice tone* proyek.
  - Menghindari konten generik dengan mempertahankan konteks satu arah (*single source of truth*).
- **Content Builder (Modular Canvas)**:
  - Kanvas penyusun konten berbasis blok modular dan raw Markdown editor.
- **Instagram & Social Media Suite**:
  - Generator Carousel multi-slide 4:5 (1080x1350) dengan preview visual dan ekspor PNG 1-klik.
  - Generator naskah Reels/Shorts/TikTok format vertikal 9:16.
  - Caption cleaner otomatis & hashtag cluster.
- **E-Book Studio**:
  - Penyusun bab, outline architect, dan compiler buku digital.
  - Ekspor instan ke format EPUB, HTML, dan Markdown.
- **Pitchdeck & Closing Suite**:
  - 10-slide VC/Client pitchdeck generator.
  - SPK (Surat Perjanjian Kerja) contract generator dan dokumen penawaran harga.
- **Omni-Channel Repurposing Engine**:
  - 1 Dokumen sumber diturunkan secara otomatis ke 5 format distribusi: Twitter/X Thread, Email Newsletter, Slide Presentasi, LinkedIn Post, dan Carousel.
- **Content Automation & Drip Publishing**:
  - Tokio background cron scheduler, drip publishing queue, dan webhook dispatcher (integrasi n8n dua arah).
- **Prompt Studio**:
  - Laboratorium kurasi prompt terstruktur untuk konten teks, media visual, dan engineering (PRD, task, development, QA audit).
- **CADS v1.0 Standard & Floating Smart Card AI**:
  - Mengikuti standar desain CADS v1.0 (*Dark Obsidian*, aksen *Violet*, SvelteKit native routing).

---

## 🏗️ Arsitektur Teknologi

- **Core & Backend**: Rust 2024, Tauri v2.
- **Frontend**: Svelte 5 (Runes), Tailwind CSS v4, CADS v1.0 Tokens.
- **Database**: SQLite lokal dengan skema UUIDv7, sync-ready change logs.
- **Automation**: Tokio cron scheduler & local HTTP inbound webhook server.

---

## 🚀 Panduan Menjalankan

### Development
```bash
cd ~/Product/castudio
cargo tauri dev
```

### Build Rilis
```bash
cargo tauri build
```
EOF
