<script lang="ts">
  import { page } from "$app/stores";
  import {
    isSidebarOpen,
    selectedNoteId,
    createEmptyNote,
    deleteNote
  } from "$lib/store";
  import NoteList from "$lib/components/NoteList.svelte";

  // Sidebar Modes
  type SidebarMode = "root" | "list";
  let sidebarMode: SidebarMode = "root";

  // Navigation Links
  const navLinks = [
    { href: "/notes", label: "Notes", icon: "N", context: "Notes" },
    { href: "/daily", label: "Daily", icon: "D", context: "Daily" },
    { href: "/projects", label: "Projects", icon: "P", context: "Projects" },
    { href: "/ideas", label: "Ideas", icon: "I", context: "Ideas" }
  ];

  $: currentPath = $page.url.pathname;

  // Auto-enter list mode when route changes
  $: if (currentPath !== "/") {
    sidebarMode = "list";
  }

  $: activeLink = navLinks.find(l => currentPath.startsWith(l.href));
  $: contextLabel = activeLink?.context ?? "";

  function goBack() {
    sidebarMode = "root";
  }

  function enterListMode() {
    sidebarMode = "list";
  }

  function toggleSidebar() {
    isSidebarOpen.update(v => !v);
  }

  // ✅ Create + persist note
  async function create() {
    const note = await createEmptyNote();

    if (currentPath.startsWith("/daily")) {
      note.title = new Date().toDateString();
      note.status = "active";
    } else if (currentPath.startsWith("/projects")) {
      note.title = "Project: ";
      note.status = "active";
    } else if (currentPath.startsWith("/ideas")) {
      note.status = "active";
    } else {
      note.status = "inbox";
    }
  }

  // ✅ Delete selected note (header-only)
  async function removeSelected() {
    if (!$selectedNoteId) return;

    const confirmed = confirm("Delete this note?");
    if (!confirmed) return;

    try {
      await deleteNote($selectedNoteId);
      selectedNoteId.set(null);
    } catch (error) {
      console.error("Error deleting note:", error);
    }
  }

  // Filter logic per context
  $: filterFn = (n: any) => {
    if (currentPath.startsWith("/notes")) return n.status === "inbox";
    if (currentPath.startsWith("/daily")) return !isNaN(Date.parse(n.title));
    if (currentPath.startsWith("/projects")) return n.title.startsWith("Project:");
    if (currentPath.startsWith("/ideas"))
      return (
        n.status === "active" &&
        !n.title.startsWith("Project:") &&
        isNaN(Date.parse(n.title))
      );
    return true;
  };
</script>

<div class="nav-container">
  {#if sidebarMode === "root"}
    <!-- ROOT MENU MODE -->
    <div class="menu-stack">
      {#each navLinks as link}
        <a
          href={link.href}
          class="menu-item"
          class:active={currentPath.startsWith(link.href)}
          on:click={() => enterListMode()}
        >
          <span class="icon">{link.icon}</span>
          {#if $isSidebarOpen}
            <span class="label">{link.label}</span>
          {/if}
        </a>
      {/each}
    </div>

    <div class="menu-footer">
      <button
        class="toggle-btn"
        on:click={toggleSidebar}
        title="Toggle Sidebar"
      >
        {$isSidebarOpen ? "«" : "»"}
      </button>
    </div>

  {:else}
    <!-- LIST MODE -->
    <header class="list-header" class:collapsed={!$isSidebarOpen}>
      <button class="back-btn" on:click={goBack}>←</button>

      {#if $isSidebarOpen}
        <span class="context-title">{contextLabel}</span>

        <div class="header-actions">
          <button class="add-btn" on:click={create} title="New note">+</button>

          <button
            class="delete-btn"
            on:click={removeSelected}
            disabled={!$selectedNoteId}
            title="Delete note"
          >
            🗑️
          </button>
        </div>
      {/if}
    </header>

    {#if $isSidebarOpen}
      <div class="list-wrapper">
        <NoteList {filterFn} />
      </div>
    {/if}
  {/if}
</div>

<style>
  .nav-container {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  /* ROOT MENU */
  .menu-stack {
    flex: 1;
    padding: 12px 8px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .menu-item {
    display: flex;
    align-items: center;
    gap: 10px;
    text-decoration: none;
    color: var(--text-muted);
    padding: 8px;
    border-radius: 6px;
    height: 36px;
  }

  .menu-item:hover {
    background-color: var(--bg-panel);
    color: var(--text-main);
  }

  .menu-item.active {
    background-color: var(--bg-input);
    color: var(--text-main);
  }

  .icon {
    width: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    font-size: 12px;
  }

  .label {
    font-size: 13px;
    white-space: nowrap;
  }

  .menu-footer {
    padding: 12px;
    border-top: 1px solid var(--border-subtle);
  }

  .toggle-btn {
    width: 100%;
    height: 30px;
    font-size: 16px;
    color: var(--text-faint);
  }

  /* LIST MODE */
  .list-header {
    height: 48px;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 12px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .list-header.collapsed {
    justify-content: center;
  }

  .context-title {
    flex: 1;
    font-size: 13px;
    font-weight: 600;
    text-align: center;
  }

  .header-actions {
    display: flex;
    gap: 6px;
  }

  .back-btn,
  .add-btn,
  .delete-btn {
    width: 28px;
    height: 28px;
    border-radius: 4px;
  }

  .delete-btn:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }

  .back-btn:hover,
  .add-btn:hover,
  .delete-btn:hover:not(:disabled) {
    background-color: var(--bg-panel);
  }

  .list-wrapper {
    flex: 1;
    overflow: hidden;
  }
</style>
