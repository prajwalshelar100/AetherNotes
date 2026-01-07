import { writable, derived } from "svelte/store";
import type { Note } from "./types";
import { getNotes, createNote, saveNote } from "./api";

// Core data
export const allNotes = writable<Note[]>([]);
export const selectedNoteId = writable<string | null>(null);
export const searchQuery = writable<string>("");

// Load notes from backend on app start
export async function loadNotes() {
  const notes = await getNotes();
  allNotes.set(notes);
}

// Derived state
export const activeNote = derived(
  [allNotes, selectedNoteId],
  ([$notes, $id]) => $notes.find((n) => n.id === $id) || null
);

// UI state
export const isSidebarOpen = writable<boolean>(true);
export const currentContext = writable<
  "notes" | "ideas" | "projects" | "daily"
>("notes");

// Factory: create + persist a new note
export async function createEmptyNote(): Promise<Note> {
  const now = Date.now();

  const note: Note = {
    id: crypto.randomUUID(),
    title: "",
    content: "",
    type: "note",
    links: [],
    status: "inbox",
    created_at: now,
    updated_at: now,
  };

  // Persist immediately
  await createNote(note);

  // Update local store
  allNotes.update((notes) => [note, ...notes]);
  selectedNoteId.set(note.id);

  return note;
}

// Autosave helper (called from Editor)
export async function persistNote(note: Note) {
  note.updated_at = Date.now();
  await saveNote(note);

  allNotes.update((notes) =>
    notes.map((n) => (n.id === note.id ? note : n))
  );
}
