// src/lib/api.ts
import { invoke } from "@tauri-apps/api/core";
import type { Note } from "./types";

export async function getNotes(): Promise<Note[]> {
  return await invoke<Note[]>("list_notes");
}

// TEMPORARY UI-ONLY STUB
// Do NOT persist until update_note exists in Rust
export async function saveNote(_note: Note): Promise<void> {
  return;
}
