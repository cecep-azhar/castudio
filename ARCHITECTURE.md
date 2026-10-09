# CAStudio Architecture

## 1. Overview
CAStudio adalah studio rekayasa konten otonom lokal (local-first autonomous content engineering studio). Berbasis pada paradigma "Single Source of Truth (Project Bible) to Omnichannel Outputs".

## 2. Diagram Arsitektur

```
+-------------------------------------------------------------+
|                CADS v1.0 UI (Svelte 5 Runes)                |
|  - SvelteKit Native Routing:                                |
|    /              -> Manajemen Proyek & Project Bible       |
|    /content       -> Modular Block Content Builder          |
|    /instagram     -> Carousel 4:5 & Reels Generator         |
|    /ebook         -> E-Book Studio & EPUB Compiler          |
|    /pitchdeck     -> 10-Slide Pitchdeck & SPK Closing       |
|    /repurpose     -> Omni-Repurpose 1-to-5 Engine           |
|    /automation    -> Drip Publishing & Tokio Cron Monitor   |
|    /prompt-studio -> Engineering & Content Prompt Hub       |
|    /settings      -> AI Model Config & GCC Billing Keys     |
|  - Floating Smart Card AI Assistant                         |
+-------------------------------------------------------------+
                             |  IPC (Tauri Commands)
+-------------------------------------------------------------+
|                    Tauri v2 Application                     |
|  - Window Controls, Frameless Shell, Menus                  |
|  - Command Dispatcher (`castudio-app`)                      |
+-------------------------------------------------------------+
                             |
+-------------------------------------------------------------+
|                  castudio-core (Rust 2024)                  |
|  - Project Bible & Document Storage Engine                  |
|  - Repurposing Engine (Multi-channel derivation)            |
|  - Automation & Tokio Cron Scheduler Engine                 |
|  - Inbound n8n Webhook Server & Outbound HTTP Dispatcher    |
|  - Local SQLite Store (UUIDv7, Sync-ready triggers)         |
+-------------------------------------------------------------+
```

## 3. Desain Komponen
- **Repurposing Engine**: Memproses teks input dan memecahnya sesuai template channel tujuan.
- **Automation Engine**: Menjalankan thread background Tokio Cron untuk memeriksa jadwal penerbitan dan memanggil n8n/HTTP endpoint.
- **Design Tokens**: Mengikuti CADS v1.0, aksen Violet `#8b5cf6`, latar belakang `#0A0A0C`.
