// src/lib/api.ts
import { invoke } from "@tauri-apps/api/core";
import type { Note } from "./types";

export async function getNotes(): Promise<Note[]> {
  return await invoke<Note[]>("list_notes");
}

export async function createNote(note: Note): Promise<void> {
  await invoke("create_note", { note });
}

export async function saveNote(note: Note): Promise<void> {
  await invoke("update_note", { note });
}

export async function deleteNote(id: string): Promise<void> {
  await invoke("delete_note", {
    noteId: id, // Tauri converts snake_case to camelCase
  });
}
