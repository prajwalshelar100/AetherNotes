<script lang="ts">
  import { activeNote, allNotes } from "$lib/store";
  import { saveNote } from "$lib/api";
  import { onDestroy } from "svelte";
  import HandwritingLayer from "./HandwritingLayer.svelte";

  let saveTimer: ReturnType<typeof setTimeout>;

  function update(field: "title" | "content", value: string) {
    if (!$activeNote) return;

    allNotes.update((notes) => {
      const idx = notes.findIndex((n) => n.id === $activeNote!.id);
      if (idx > -1) {
        notes[idx] = {
          ...notes[idx],
          [field]: value,
          updated_at: Date.now()
        };

        clearTimeout(saveTimer);
        saveTimer = setTimeout(() => {
          saveNote(notes[idx]);
        }, 1000);
      }
      return notes;
    });
  }

  onDestroy(() => {
    clearTimeout(saveTimer);
  });
</script>

<div class="editor-scroller">
  {#if $activeNote}
    <div class="document-column">
      <!-- Title -->
      <input
        class="document-title"
        value={$activeNote.title}
        placeholder="Untitled"
        on:input={(e) => update("title", e.currentTarget.value)}
      />

      <!-- Meta -->
      <div class="document-meta">
        <span>{$activeNote.status}</span>
        <span class="separator">•</span>
        <span>{new Date($activeNote.updated_at).toLocaleString()}</span>
      </div>

      <!-- Writing Surface -->
      <div class="surface-wrapper">
        <HandwritingLayer />

        <textarea
          class="document-content"
          value={$activeNote.content}
          placeholder="Start writing..."
          spellcheck="false"
          on:input={(e) => update("content", e.currentTarget.value)}
        ></textarea>
      </div>
    </div>
  {:else}
    <div class="empty-state">
      <p>Select a note to begin.</p>
    </div>
  {/if}
</div>

<style>
  .editor-scroller {
    width: 100%;
    height: 100%;
    overflow-y: auto;
    overflow-x: hidden;
    position: relative;
    background-color: var(--bg-app);
    cursor: text;
  }

  .document-column {
    max-width: 750px;
    margin: 0 auto;
    padding: 60px 40px 120px 40px;
    min-height: 100%;
    display: flex;
    flex-direction: column;
  }

  .document-title {
    font-size: 36px;
    font-weight: 700;
    color: var(--text-main);
    width: 100%;
    background: transparent;
    border: none;
    outline: none;
    margin-bottom: 12px;
    line-height: 1.2;
  }

  .document-title::placeholder {
    color: var(--border-focus);
    opacity: 0.5;
  }

  .document-meta {
    font-size: 13px;
    color: var(--text-faint);
    margin-bottom: 48px;
    display: flex;
    align-items: center;
    text-transform: capitalize;
  }

  .separator {
    margin: 0 8px;
    opacity: 0.5;
  }

  .surface-wrapper {
    position: relative;
    flex: 1;
    display: flex;
  }

  .document-content {
    width: 100%;
    min-height: 60vh;
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    resize: none;
    font-family: var(--font-editor);
    font-size: 19px;
    line-height: 1.65;
    color: var(--text-main);
    position: relative;
    z-index: 5;
  }

  .document-content::placeholder {
    color: var(--text-muted);
    font-style: italic;
    opacity: 0.6;
  }

  .empty-state {
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-faint);
    font-size: 14px;
  }
</style>
