# AetherNotes

> **Private. Offline. Personal.**
>
> AetherNotes is a local-first, encrypted personal thinking environment designed for long-term idea development — not productivity metrics or cloud sync.

---

## ✨ What is AetherNotes?

**AetherNotes** is a **single-user, offline-only application** for capturing, developing, and connecting ideas over time.

It is built as a **thinking space**, not a task manager or collaboration tool.  
There are no accounts, no cloud services, no telemetry, and no background sync.

You own:
- The software
- The data
- The thinking process

---

## 🧠 Core Principles

- 🔒 **Offline-only by design**
- 🧑‍💻 **Single-user system**
- 🗄️ **Encrypted internal storage**
- ✍️ **Markdown-based writing**
- 🧩 **Visual + writing hybrid UI**
- 🔗 **Flat pages with wiki-style links**
- 🌱 **Designed for long-term use**
- 🌍 **Open-source code, private data**

### What This App Is NOT
- ❌ Cloud-based
- ❌ Collaborative
- ❌ A task manager
- ❌ AI-driven (v1)
- ❌ Plugin-based (v1)

---

## 🖥️ Target Platforms

| Platform | Status |
|--------|--------|
| Linux (Pop!_OS) | Primary |
| Android | Planned |
| iPadOS | Planned |

All platforms share:
- The same codebase
- The same encrypted data format
- The same mental model

---

## 🧱 Tech Stack

### Frontend
- **SvelteKit**
- **TypeScript**
- **Vite**

### Backend
- **Rust**
- **Tauri**
- **Encrypted SQLite** (internal)

### Why This Stack?
- **Tauri** → small, secure, no bundled browser
- **Rust** → memory safety + cryptography
- **SvelteKit** → minimal runtime, clean UI model
- **TypeScript** → correctness as the app grows

---

## 🗂️ Project Structure

```text
aethernotes/
├── src/
│   ├── routes/
│   │   ├── +page.svelte        # Main UI page
│   │   └── +layout.ts
│   ├── app.html                # HTML shell
│   └── lib/
│       ├── components/
│       ├── stores/
│       └── utils/
│
├── src-tauri/
│   ├── src/
│   │   ├── main.rs             # Rust entry point
│   │   ├── commands.rs         # Tauri IPC commands
│   │   ├── vault/              # encryption + storage
│   │   └── notes/              # domain models
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── static/
├── package.json
├── svelte.config.js
├── tsconfig.json
├── README.md
└── specificationsv2.md
```

> ⚠️ This is a **SvelteKit** project — there is no `App.svelte`.

---

## 🧩 Mental Model

### Note Types
- **Note** → raw capture
- **Idea** → atomic thought
- **Project** → container of effort
- **Daily** → chronological thinking

### Linking
- Wiki-style `[[links]]`
- Visual connections
- No enforced hierarchy

---

## 🔐 Security Model

- AES-256 encrypted storage
- Password-derived encryption key
- No network APIs enabled
- No telemetry or analytics
- Manual encrypted export only

---

## 🔄 Core Workflows

### Capture
1. Open the app
2. Write a quick note
3. Save instantly

### Thinking
1. Convert note → idea
2. Write in Markdown
3. Link related thoughts

### Project Work
1. Open a project
2. Review current state
3. Add progress notes
4. Rearrange ideas visually

---

## 🚧 Development Status

- ✅ Tauri + SvelteKit environment working
- ✅ Desktop app running locally
- 🚧 Core domain logic in progress
- 🚧 UI foundation under active development

---

## 🛠️ Development Setup

```bash
npm install
npm run tauri dev
```

> Linux requires WebKitGTK development libraries.

---

## 📜 Philosophy

This project favors:
- Clarity over features
- Longevity over trends
- Ownership over convenience

AetherNotes is meant to be **lived in**, slowly shaped by real usage.

---

## 📄 License

Open-source (license to be finalized).  
User data is always private and never shared.

---

## 🧭 Roadmap (V1)

- Encrypted vault
- Note / Idea / Project CRUD
- Markdown editor
- Visual project boards
- Manual encrypted export

---

## ✨ Final Note

This is not just an app.  
It is a **personal thinking environment**.

Build slowly.  
Keep it simple.  
Let structure emerge.
