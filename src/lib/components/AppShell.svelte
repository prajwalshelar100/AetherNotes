<script lang="ts">
  import { isSidebarOpen } from "$lib/store";
</script>

<div class="shell">
  <!-- Window Drag Region -->
  <div class="titlebar" data-tauri-drag-region></div>

  <div class="layout" class:sidebar-closed={!$isSidebarOpen}>
    <aside class="sidebar">
      <slot name="sidebar" />
    </aside>

    <main class="content">
      <slot />
    </main>
  </div>
</div>

<style>
  .shell {
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 100vw;
    background-color: var(--bg-app);
  }

  .titlebar {
    height: 30px;
    width: 100%;
    position: absolute;
    top: 0;
    left: 0;
    z-index: 1000;
  }

  .layout {
    display: grid;
    grid-template-columns: var(--sidebar-width) 1fr;
    flex: 1;
    height: 100%;
    padding-top: 30px;
    transition: grid-template-columns 0.3s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .layout.sidebar-closed {
    grid-template-columns: 50px 1fr;
  }

  .sidebar {
    background-color: var(--bg-sidebar);
    border-right: 1px solid var(--border-subtle);
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .content {
    background-color: var(--bg-app);
    overflow: hidden;
    display: flex;
    flex-direction: column;
    position: relative;
  }
</style>
