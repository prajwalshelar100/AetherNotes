# AetherNotes

> **Private. Offline. Encrypted.**
>
> AetherNotes is a local-first, encrypted personal notes application designed for long-term thinking — not cloud sync, collaboration, or productivity dashboards.

---

## ✨ What Is AetherNotes?

**AetherNotes** is a **single-user, offline-only desktop application** for writing, organizing, and revisiting notes over time.

It is intentionally:
- Calm
- Private
- Ownership-focused

There are **no accounts, no cloud services, no telemetry, and no background sync**.  
Everything lives on **your machine**, encrypted at rest.

You own:
- The data
- The storage
- The software
- The thinking process

---

## 🧠 Core Principles

- 🔒 **Offline-only by design**
- 🧑‍💻 **Single-user system**
- 🗄️ **Encrypted local storage**
- ✍️ **Focused writing experience**
- 🧭 **Context-based notes (Notes / Daily / Projects / Ideas)**
- 🌱 **Built for long-term use**
- 🌍 **Open-source code, private data**

### What This App Is NOT

- ❌ Cloud-based  
- ❌ Collaborative  
- ❌ Account-based  
- ❌ AI-driven (v1)  
- ❌ Plugin-driven (v1)  
- ❌ A task manager  

---

## 🖥️ Supported Platforms

| Platform | Status |
|--------|--------|
| **Linux** | ✅ Supported (deb, rpm) |
| **Windows** | ⏳ Planned |
| **Android** | ⏳ Planned (Tauri v2) |
| **iPadOS** | ⏳ Planned (Tauri v2) |

> Mobile platforms are a **future milestone**, not part of v1.

---

## 🧱 Tech Stack

### Frontend
- **SvelteKit**
- **TypeScript**
- **Vite**
- Store-based state management

### Backend
- **Rust**
- **Tauri**
- **Encrypted SQLite**

### Why This Stack?

- **Tauri** → small binaries, no bundled browser, native IPC  
- **Rust** → memory safety, encryption correctness  
- **SvelteKit** → minimal runtime, clean UI model  
- **TypeScript** → correctness as the app evolves  

---

## 🗂️ Project Structure

```text
aethernotes/
├── src/
│   ├── routes/
│   │   ├── +page.svelte        # Main UI
│   │   └── +layout.ts
│   ├── app.html
│   └── lib/
│       ├── components/        # UI components
│       ├── store.ts           # App state
│       ├── api.ts             # Tauri IPC
│       └── types.ts
│
├── src-tauri/
│   ├── src/
│   │   ├── main.rs            # App entry
│   │   ├── commands.rs        # Tauri commands
│   │   ├── notes/             # Domain models + repository
│   │   └── vault/             # Encryption, DB lifecycle, backups
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── static/
├── package.json
├── README.md
└── specifications.md
