<script>
  import { onMount, onDestroy } from 'svelte';
  import { fetchProjects, fetchProjectOverview } from './lib/api.js';
  import ProjectBoard from './ProjectBoard.svelte';

  let projects = $state([]);
  let overviews = $state({});
  let loading = $state(true);
  let error = $state(null);
  let collapsedProjects = $state(JSON.parse(localStorage.getItem('uvp-kanban-collapsed') || '{}'));
  let refreshTimer = $state(null);
  let lastRefresh = $state(null);

  function saveCollapsed() {
    localStorage.setItem('uvp-kanban-collapsed', JSON.stringify(collapsedProjects));
  }

  function toggleCollapse(id) {
    collapsedProjects[id] = !collapsedProjects[id];
    collapsedProjects = { ...collapsedProjects };
    saveCollapsed();
  }

  async function loadAll() {
    try {
      projects = await fetchProjects();
      const results = await Promise.all(
        projects
          .filter(p => p.available)
          .map(p => fetchProjectOverview(p.id).then(o => [p.id, o]))
      );
      for (const [id, overview] of results) {
        overviews[id] = overview;
      }
      lastRefresh = new Date();
    } catch (e) {
      error = e.message;
    } finally {
      loading = false;
    }
  }

  async function refreshProject(id) {
    try {
      overviews[id] = await fetchProjectOverview(id);
      overviews = { ...overviews };
    } catch (e) {
      console.error('Refresh failed:', e);
    }
  }

  async function refreshAll() {
    for (const p of projects.filter(p => p.available)) {
      try {
        overviews[p.id] = await fetchProjectOverview(p.id);
      } catch (_) {}
    }
    overviews = { ...overviews };
    lastRefresh = new Date();
  }

  // P6: Auto-refresh every 10 minutes on the clock (e.g. :00, :10, :20, :30, :40, :50)
  function scheduleNextRefresh() {
    const now = new Date();
    const mins = now.getMinutes();
    const nextTen = Math.ceil((mins + 1) / 10) * 10;
    const msUntilNext = ((nextTen - mins) * 60 - now.getSeconds()) * 1000 - now.getMilliseconds();
    refreshTimer = setTimeout(async () => {
      await refreshAll();
      scheduleNextRefresh();
    }, msUntilNext);
  }

  // P3: Global keyboard shortcuts
  function handleKeydown(e) {
    if (e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA') return;

    if (e.key === 'Escape') {
      // Broadcast escape to clear pinned state (handled by ProjectBoard)
      window.dispatchEvent(new CustomEvent('kanban-escape'));
    } else if (e.key === '/') {
      e.preventDefault();
      // Focus the first visible TODO input
      const input = document.querySelector('[data-todo-input]');
      if (input) input.focus();
    } else if (e.key >= '1' && e.key <= '9') {
      const idx = parseInt(e.key) - 1;
      const boards = document.querySelectorAll('[data-project-board]');
      if (boards[idx]) {
        boards[idx].scrollIntoView({ behavior: 'smooth', block: 'start' });
      }
    }
  }

  onMount(async () => {
    await loadAll();
    scheduleNextRefresh();
    window.addEventListener('keydown', handleKeydown);
  });

  onDestroy(() => {
    if (refreshTimer) clearTimeout(refreshTimer);
    window.removeEventListener('keydown', handleKeydown);
  });

  function pendingCount(projectId) {
    const o = overviews[projectId];
    if (!o) return 0;
    return o.todos.filter(t => !t.done).length;
  }

  function formatTime(date) {
    if (!date) return '';
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  }
</script>

<div class="min-h-screen flex flex-col">
  <!-- Header -->
  <header class="sticky top-0 z-50 border-b border-[var(--color-border)] bg-[var(--color-bg)]/80 backdrop-blur-md">
    <div class="mx-auto px-6 py-3 flex items-center justify-between">
      <div class="flex items-center gap-3">
        <div class="w-7 h-7 rounded-[var(--radius-sm)] bg-[var(--color-primary)] flex items-center justify-center text-white text-xs font-bold">U</div>
        <h1 class="text-base font-semibold tracking-tight">UVP Kanban</h1>
      </div>
      <div class="flex items-center gap-4">
        {#if lastRefresh}
          <span class="text-[10px] text-[var(--color-text-muted)]">
            refreshed {formatTime(lastRefresh)}
          </span>
        {/if}
        <span class="text-xs text-[var(--color-text-muted)]">
          {projects.filter(p => p.available).length} project{projects.filter(p => p.available).length !== 1 ? 's' : ''}
        </span>
        <button
          class="text-[10px] text-[var(--color-text-muted)] hover:text-[var(--color-text)] px-2 py-1 rounded-[var(--radius-sm)] hover:bg-[var(--color-surface-hover)] transition-colors"
          onclick={refreshAll}
        >↻ Refresh</button>
      </div>
    </div>
  </header>

  <!-- Content -->
  <main class="flex-1 px-6 py-6 space-y-4">
    {#if loading}
      <div class="flex items-center justify-center py-32">
        <div class="text-center space-y-3">
          <div class="w-6 h-6 border-2 border-[var(--color-primary)] border-t-transparent rounded-full animate-spin mx-auto"></div>
          <p class="text-sm text-[var(--color-text-muted)]">Loading projects...</p>
        </div>
      </div>
    {:else if error}
      <div class="rounded-[var(--radius-lg)] p-6 text-center border border-[var(--color-danger-muted)] bg-[var(--color-danger-muted)]/10">
        <p class="text-sm text-[var(--color-danger)]">{error}</p>
      </div>
    {:else if projects.length === 0}
      <div class="rounded-[var(--radius-lg)] border border-[var(--color-border)] bg-[var(--color-surface)] p-16 text-center">
        <p class="text-base mb-2">No projects registered</p>
        <p class="text-sm text-[var(--color-text-muted)]">
          Add projects in <code class="px-1.5 py-0.5 rounded-[var(--radius-sm)] bg-[var(--color-surface-hover)] text-[var(--color-text-secondary)] font-mono text-xs">~/.uvp/uvp.toml</code>
        </p>
      </div>
    {:else}
      {#each projects as project (project.id)}
        {#if project.available && overviews[project.id]}
          <!-- P5: Collapsible project -->
          <div data-project-board>
            {#if collapsedProjects[project.id]}
              <!-- Collapsed state -->
              <button
                class="w-full rounded-[var(--radius-lg)] border border-[var(--color-border)] bg-[var(--color-surface)] px-5 py-3 flex items-center gap-3 hover:bg-[var(--color-surface-hover)] transition-colors text-left"
                onclick={() => toggleCollapse(project.id)}
              >
                <svg class="w-3.5 h-3.5 text-[var(--color-text-muted)]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7"/>
                </svg>
                <div class="w-2 h-2 rounded-full bg-[var(--color-success)]"></div>
                <span class="text-sm font-semibold">{project.name}</span>
                {#if project.version}
                  <span class="text-[10px] font-mono text-[var(--color-text-muted)]">v{project.version}</span>
                {/if}
                <span class="text-xs text-[var(--color-text-muted)] ml-auto">
                  {pendingCount(project.id)} pending
                </span>
              </button>
            {:else}
              <!-- Expanded state -->
              <ProjectBoard
                overview={overviews[project.id]}
                onRefresh={() => refreshProject(project.id)}
                onCollapse={() => toggleCollapse(project.id)}
              />
            {/if}
          </div>
        {:else if !project.available}
          <div data-project-board class="rounded-[var(--radius-lg)] border border-[var(--color-border)] bg-[var(--color-surface)] p-4 flex items-center gap-3 opacity-50">
            <div class="w-2 h-2 rounded-full bg-[var(--color-warning)]"></div>
            <span class="text-sm font-medium">{project.name}</span>
            <span class="text-xs text-[var(--color-text-muted)]">unavailable</span>
          </div>
        {/if}
      {/each}
    {/if}
  </main>

  <!-- Footer hint -->
  <footer class="border-t border-[var(--color-border)] px-6 py-2 text-center">
    <span class="text-[10px] text-[var(--color-text-muted)]">
      <kbd class="px-1 py-0.5 rounded bg-[var(--color-surface-hover)] font-mono">Esc</kbd> clear highlight
      <span class="mx-2">·</span>
      <kbd class="px-1 py-0.5 rounded bg-[var(--color-surface-hover)] font-mono">/</kbd> focus todo
      <span class="mx-2">·</span>
      <kbd class="px-1 py-0.5 rounded bg-[var(--color-surface-hover)] font-mono">1-9</kbd> jump to project
    </span>
  </footer>
</div>
