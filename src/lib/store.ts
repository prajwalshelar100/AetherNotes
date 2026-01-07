// src/lib/store.ts
import { writable, derived } from "svelte/store";
import type { Note } from "./types";

export const allNotes = writable<Note[]>([]);
export const selectedNoteId = writable<string | null>(null);
export const searchQuery = writable<string>("");

export const activeNote = derived(
  [allNotes, selectedNoteId],
  ([$notes, $id]) => $notes.find((n) => n.id === $id) || null
);

export const isSidebarOpen = writable<boolean>(true);
export const currentContext = writable<string>("notes");

export function createEmptyNote(): Note {
  const now = Date.now();
  return {
    id: crypto.randomUUID(),
    title: "",
    content: "",
    type: "note",
    links: [],
    status: "inbox",
    created_at: now,
    updated_at: now,
  };
}
