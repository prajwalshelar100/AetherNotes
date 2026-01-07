<script lang="ts">
  import { allNotes, selectedNoteId } from "$lib/store";
  import type { Note } from "$lib/types";

  export let filterFn: (n: Note) => boolean = () => true;

  $: filteredNotes = [...$allNotes]
    .filter(filterFn)
    .sort((a, b) => b.updated_at - a.updated_at);

  function select(id: string) {
    selectedNoteId.set(id);
  }

  function formatDate(ts: number) {
    return new Date(ts).toLocaleDateString(undefined, {
      month: "short",
      day: "numeric",
    });
  }
</script>

<div class="list-container">
  {#if filteredNotes.length === 0}
    <div class="empty">No notes found.</div>
  {:else}
    <ul class="note-list">
      {#each filteredNotes as note}
        <li
          class:selected={$selectedNoteId === note.id}
          on:click={() => select(note.id)}
        >
          <div class="note-title">
            {note.title || "Untitled"}
          </div>

          <div class="note-meta">
            <span class="date">{formatDate(note.updated_at)}</span>
            <span class="preview">
              {note.content.slice(0, 30).replace(/\n/g, " ") || "No content"}
            </span>
          </div>
        </li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  .list-container {
    height: 100%;
    overflow-y: auto;
    overflow-x: hidden;
    padding: var(--spacing-sm);
    scroll-behavior: smooth;
  }

  .empty {
    padding: var(--spacing-md);
    color: var(--text-faint);
    text-align: center;
    font-size: 13px;
  }

  .note-list {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  li {
    padding: 10px 12px;
    margin-bottom: 4px;
    border-radius: 4px;
    cursor: pointer;
    border: 1px solid transparent;
  }

  li:hover {
    background-color: var(--bg-panel);
  }

  li.selected {
    background-color: var(--bg-input);
    border-color: var(--border-subtle);
  }

  .note-title {
    font-weight: 500;
    font-size: 14px;
    margin-bottom: 4px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .note-meta {
    display: flex;
    justify-content: space-between;
    font-size: 11px;
    color: var(--text-muted);
    gap: 8px;
  }

  .preview {
    opacity: 0.7;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 60%;
  }
</style>
