<script lang="ts">
  import "../app.css";
  import { onMount } from "svelte";
  import { allNotes } from "$lib/store";
  import { getNotes } from "$lib/api";
  import AppShell from "$lib/components/AppShell.svelte";
  import Navigation from "$lib/components/Navigation.svelte";

  let initialized = false;

  onMount(async () => {
    if (initialized) return;
    initialized = true;
    try {
      const notes = await getNotes();
      allNotes.set(notes);
    } catch (e) {
      console.error("Failed to load notes", e);
    }
  });
</script>

<AppShell>
  <div slot="sidebar" class="sidebar-inner">
    <div class="brand">AetherNotes</div>
    <Navigation />
  </div>

  <slot />
</AppShell>

<style>
  .sidebar-inner {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .brand {
    padding: 20px 24px;
    font-weight: 600;
    letter-spacing: 0.4px;
    color: var(--text-main);
    opacity: 0.85;
  }
</style>
