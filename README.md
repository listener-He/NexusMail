# NexusMail

**NexusMail** is a Local-First, Zero-Trust, High-Performance email client designed with a chat-like interface.

## Tech Stack

- **Core**: Tauri 2.0 (Rust)
- **Frontend**: React 19 + TypeScript + Vite
- **Styling**: Tailwind CSS (Lumina Design System)
- **Database**: SQLite + SQLCipher (Encrypted)
- **Search**: Sonic (Rust-based)

## Project Structure

- `src/` - Frontend source code (React)
  - `components/` - UI Components (Atoms/Molecules)
  - `features/` - Business Logic Modules
  - `lib/` - Core Infrastructure
- `src-tauri/` - Backend source code (Rust)
  - `src/database/` - DB Layer
  - `src/engine/` - Nexus Workflow Engine
  - `src/security/` - Zero-Trust Logic

## Getting Started

1. **Install Dependencies**:
   ```bash
   npm install
   ```

2. **Run Development Server**:
   ```bash
   npm run dev
   # Or run with Tauri
   npm run tauri dev
   ```

3. **Build**:
   ```bash
   npm run tauri build
   ```

## Design Philosophy

- **Local-First**: All data lives on your device.
- **Zero-Trust**: No default permissions for third-party APIs.
- **Chat-Like**: Emails are treated as conversations.
