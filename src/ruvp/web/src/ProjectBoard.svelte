<script>
  import { onMount, onDestroy, tick } from 'svelte';
  import { addTodo, patchTodo, deleteTodo } from './lib/api.js';
  import EdgeCanvas from './EdgeCanvas.svelte';

  let { overview, onRefresh, onCollapse } = $props();

  let hoveredNode = $state(null);
  let pinnedNode = $state(null);
  let newTodoText = $state('');
  let adding = $state(false);
  let showAllFeatures = $state(false);
  let showCompletedRoadmap = $state(false);
  let boardEl = $state(null);
  let gridEl = $state(null);
  let nodePositions = $state({});
  let todoInputEl = $state(null);

  let activeNode = $derived(pinnedNode || hoveredNode);

  // P1: Features filtering
  const TERMINAL_STATUSES = ['implemented', 'closed', 'deprecated', 'removed'];
  let activeFeatures = $derived(overview.features.filter(f => !TERMINAL_STATUSES.includes(f.status)));
  let terminalFeatures = $derived(overview.features.filter(f => TERMINAL_STATUSES.includes(f.status)));
  let visibleFeatures = $derived((showAllFeatures ? overview.features : activeFeatures).toReversed());

  // Roadmap filtering: separate completed (all linked features done) from active
  let indexedRoadmap = $derived(overview.roadmap.map((item, idx) => ({ ...item, _idx: idx })));
  let activeRoadmap = $derived(indexedRoadmap.filter(item => {
    const p = roadmapProgress(item);
    return !p || !p.complete;
  }));
  let completedRoadmap = $derived(indexedRoadmap.filter(item => {
    const p = roadmapProgress(item);
    return p && p.complete;
  }));

  // P4: Roadmap progress
  function roadmapProgress(item) {
    if (!item.linked_features || item.linked_features.length === 0) return null;
    const total = item.linked_features.length;
    const done = item.linked_features.filter(fid => {
      const feat = overview.features.find(f => f.id === fid);
      return feat && TERMINAL_STATUSES.includes(feat.status);
    }).length;
    return { done, total, complete: done === total };
  }

  function getHighlightTiers(nodeKey) {
    if (!nodeKey || !overview) return { direct: new Set(), indirect: new Set() };
    const edges = overview.edges || [];

    // BFS: compute distance from the active node
    const dist = new Map();
    dist.set(nodeKey, 0);
    const queue = [nodeKey];

    while (queue.length > 0) {
      const current = queue.shift();
      const d = dist.get(current);
      for (const edge of edges) {
        const fromKey = `${edge.from_type}:${edge.from_id}`;
        const toKey = `${edge.to_type}:${edge.to_id}`;
        let neighbor = null;
        if (fromKey === current) neighbor = toKey;
        else if (toKey === current) neighbor = fromKey;
        if (neighbor && !dist.has(neighbor)) {
          dist.set(neighbor, d + 1);
          queue.push(neighbor);
        }
      }
    }

    const direct = new Set();
    const indirect = new Set();
    for (const [key, d] of dist) {
      if (d === 0) { direct.add(key); }
      else if (d === 1) { direct.add(key); }
      else { indirect.add(key); }
    }
    return { direct, indirect };
  }

  let highlightTiers = $derived(getHighlightTiers(activeNode));
  let highlightedIds = $derived(new Set([...highlightTiers.direct, ...highlightTiers.indirect]));

  function isDimmed(type, id) {
    if (!activeNode) return false;
    const key = `${type}:${id}`;
    return !highlightTiers.direct.has(key) && !highlightTiers.indirect.has(key);
  }

  function isHighlighted(type, id) {
    if (!activeNode) return false;
    return highlightTiers.direct.has(`${type}:${id}`);
  }

  function isSecondary(type, id) {
    if (!activeNode) return false;
    return highlightTiers.indirect.has(`${type}:${id}`);
  }

  function nodeClass(type, id) {
    if (isDimmed(type, id)) return 'opacity-20 scale-[0.98] transition-all duration-200';
    if (isHighlighted(type, id)) return 'ring-1 ring-[var(--color-primary)] shadow-[0_0_12px_rgba(99,102,241,0.15)] transition-all duration-200';
    if (isSecondary(type, id)) return 'ring-1 ring-[var(--color-primary)]/40 opacity-70 transition-all duration-200';
    return 'transition-all duration-200';
  }

  function stripAdrTags(text) {
    return text.replace(/\s*\[\[ADR-\d+\]\]/gi, '').trim();
  }

  function handleNodeClick(e, nodeKey) {
    if (e.target.closest('button') || e.target.closest('form') || e.target.closest('input')) return;
    pinnedNode = pinnedNode === nodeKey ? null : nodeKey;
  }

  function handleBoardClick(e) {
    if (!e.target.closest('[data-node]') && !e.target.closest('button') && !e.target.closest('form')) {
      pinnedNode = null;
    }
  }

  // P3: Listen for Escape from App
  function handleEscape() {
    pinnedNode = null;
  }

  async function handleAddTodo() {
    if (!newTodoText.trim()) return;
    adding = true;
    try {
      await addTodo(overview.project.id, newTodoText.trim());
      newTodoText = '';
      onRefresh?.();
    } finally {
      adding = false;
    }
  }

  async function handleToggleTodo(todo) {
    await patchTodo(overview.project.id, todo.id, !todo.done);
    onRefresh?.();
  }

  async function handleDeleteTodo(todo) {
    await deleteTodo(overview.project.id, todo.id);
    onRefresh?.();
  }

  function statusColor(status) {
    const map = {
      idea: 'var(--color-text-muted)',
      planned: 'var(--color-primary)',
      implementing: 'var(--color-warning)',
      verifying: 'var(--color-primary-hover)',
      verified: 'var(--color-success)',
      implemented: 'var(--color-success)',
      closed: 'var(--color-text-muted)',
      proposed: 'var(--color-warning)',
      accepted: 'var(--color-success)',
      deprecated: 'var(--color-danger)',
      superseded: 'var(--color-danger)',
    };
    return map[status] || 'var(--color-text-muted)';
  }

  // Roadmap section color mapping (near-term=green, mid-term=warning, long-term=primary)
  function sectionColor(section) {
    const s = (section || '').toLowerCase();
    if (s.includes('近期') || s.includes('near') || s.includes('short')) return { text: 'var(--color-success)', bg: 'rgba(34,197,94,0.1)', border: 'var(--color-success-muted)' };
    if (s.includes('中期') || s.includes('mid') || s.includes('medium')) return { text: 'var(--color-warning)', bg: 'rgba(245,158,11,0.1)', border: 'var(--color-warning-muted)' };
    if (s.includes('远期') || s.includes('long') || s.includes('far')) return { text: 'var(--color-primary)', bg: 'rgba(99,102,241,0.1)', border: 'var(--color-primary-muted)' };
    return { text: 'var(--color-text-muted)', bg: 'transparent', border: 'var(--color-border)' };
  }

  // FEAT tag color palette for distinguishing different FEAT IDs
  const FEAT_COLORS = [
    { bg: 'rgba(99,102,241,0.12)', text: '#818cf8' },   // indigo
    { bg: 'rgba(236,72,153,0.12)', text: '#f472b6' },   // pink
    { bg: 'rgba(34,197,94,0.12)', text: '#4ade80' },     // green
    { bg: 'rgba(245,158,11,0.12)', text: '#fbbf24' },    // amber
    { bg: 'rgba(6,182,212,0.12)', text: '#22d3ee' },     // cyan
    { bg: 'rgba(168,85,247,0.12)', text: '#c084fc' },    // purple
    { bg: 'rgba(251,146,60,0.12)', text: '#fb923c' },    // orange
    { bg: 'rgba(56,189,248,0.12)', text: '#38bdf8' },    // sky
  ];

  function featColor(featId) {
    const num = parseInt((featId || '').replace(/\D/g, ''), 10) || 0;
    return FEAT_COLORS[num % FEAT_COLORS.length];
  }

  function updateNodePositions() {
    if (!gridEl) return;
    const positions = {};
    gridEl.querySelectorAll('[data-node]').forEach(el => {
      const key = el.getAttribute('data-node');
      const rect = el.getBoundingClientRect();
      const containerRect = gridEl.getBoundingClientRect();
      positions[key] = {
        x: rect.left - containerRect.left + rect.width / 2,
        y: rect.top - containerRect.top + rect.height / 2,
        width: rect.width,
        height: rect.height,
      };
    });
    nodePositions = positions;
  }

  onMount(() => {
    tick().then(updateNodePositions);
    const observer = new ResizeObserver(updateNodePositions);
    if (gridEl) observer.observe(gridEl);
    window.addEventListener('kanban-escape', handleEscape);
    return () => {
      observer.disconnect();
      window.removeEventListener('kanban-escape', handleEscape);
    };
  });

  $effect(() => {
    if (overview || showAllFeatures || showCompletedRoadmap) {
      tick().then(updateNodePositions);
    }
  });
</script>

<div class="rounded-[var(--radius-lg)] border border-[var(--color-border)] bg-[var(--color-surface)] overflow-hidden" bind:this={boardEl}>
  <!-- Project header -->
  <div class="px-5 py-3 flex items-center gap-3 border-b border-[var(--color-border)] bg-[var(--color-bg-elevated)]">
    <!-- P5: Collapse button -->
    <button
      class="text-[var(--color-text-muted)] hover:text-[var(--color-text)] transition-colors"
      onclick={onCollapse}
      title="Collapse project"
    >
      <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
        <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7"/>
      </svg>
    </button>
    <div class="w-2 h-2 rounded-full bg-[var(--color-success)]"></div>
    <h2 class="text-sm font-semibold">{overview.project.name}</h2>
    {#if overview.project.version}
      <span class="text-xs font-mono text-[var(--color-text-muted)] px-1.5 py-0.5 rounded-[var(--radius-sm)] bg-[var(--color-surface-hover)]">
        v{overview.project.version}
      </span>
    {/if}
    {#if overview.project.description}
      <span class="text-xs text-[var(--color-text-muted)] hidden sm:inline">{overview.project.description}</span>
    {/if}
  </div>

  <!-- Board content -->
  <div class="relative p-5" bind:this={gridEl} onclick={handleBoardClick}>
    <EdgeCanvas edges={overview.edges || []} {nodePositions} hoveredNode={activeNode} {highlightedIds} {highlightTiers} />

    <div class="grid grid-cols-4 gap-10 relative z-10">
      <!-- Roadmap column (P4: progress indicators, color-coded by timeframe, completed folded) -->
      <div class="min-w-0">
        <div class="text-[11px] font-semibold uppercase tracking-widest text-[var(--color-text-muted)] mb-3 px-1">
          Roadmap <span class="text-[var(--color-text-muted)]/60">({activeRoadmap.length} active)</span>
        </div>
        <div class="space-y-2">
          {#each activeRoadmap as item}
            {@const idx = item._idx}
            {@const progress = roadmapProgress(item)}
            {@const secColor = sectionColor(item.section)}
            <div
              class="rounded-[var(--radius-md)] border bg-[var(--color-bg-elevated)] px-3 py-2.5 text-sm cursor-pointer hover:border-[var(--color-border-strong)] {nodeClass('roadmap', `roadmap-${idx}`)} border-[var(--color-border)]"
              style="border-left: 3px solid {secColor.text}; background: linear-gradient(90deg, {secColor.bg} 0%, var(--color-bg-elevated) 40%)"
              data-node="roadmap:roadmap-{idx}"
              onmouseenter={() => hoveredNode = `roadmap:roadmap-${idx}`}
              onmouseleave={() => hoveredNode = null}
              onclick={(e) => handleNodeClick(e, `roadmap:roadmap-${idx}`)}
            >
              <div class="flex items-center gap-2 mb-1">
                <div class="text-[10px] uppercase tracking-wider font-medium" style="color: {secColor.text}">{item.section}</div>
                {#if progress}
                  <span class="ml-auto text-[10px] font-mono text-[var(--color-text-muted)]">
                    {progress.done}/{progress.total}
                  </span>
                {/if}
              </div>
              <div class="text-xs leading-relaxed text-[var(--color-text-secondary)]">{item.text}</div>
              {#if progress && !progress.complete}
                <div class="mt-2 h-1 rounded-full bg-[var(--color-surface-hover)] overflow-hidden">
                  <div class="h-full rounded-full transition-all" style="width: {(progress.done / progress.total) * 100}%; background: {secColor.text}"></div>
                </div>
              {/if}
              {#if item.linked_features.length > 0}
                <div class="mt-2 flex flex-wrap gap-1">
                  {#each item.linked_features as feat}
                    {@const fc = featColor(feat)}
                    <span class="text-[10px] font-mono px-1.5 py-0.5 rounded-[var(--radius-sm)]" style="background: {fc.bg}; color: {fc.text}">{feat}</span>
                  {/each}
                </div>
              {/if}
            </div>
          {:else}
            <div class="text-xs italic text-[var(--color-text-muted)] px-1">No active roadmap items</div>
          {/each}

          <!-- Completed roadmap fold toggle -->
          {#if completedRoadmap.length > 0}
            <button
              class="w-full text-[10px] text-[var(--color-text-muted)] hover:text-[var(--color-text-secondary)] py-1.5 px-2 rounded-[var(--radius-sm)] hover:bg-[var(--color-surface-hover)] transition-colors text-left"
              onclick={() => { showCompletedRoadmap = !showCompletedRoadmap; }}
            >
              {showCompletedRoadmap ? '▾ Hide' : '▸ Show'} {completedRoadmap.length} completed
            </button>
            {#if showCompletedRoadmap}
              {#each completedRoadmap as item}
                {@const idx = item._idx}
                {@const secColor = sectionColor(item.section)}
                <div
                  class="rounded-[var(--radius-md)] border border-[var(--color-success-muted)] bg-[var(--color-bg-elevated)] px-3 py-2.5 text-sm cursor-pointer opacity-60 hover:opacity-80 {nodeClass('roadmap', `roadmap-${idx}`)}"
                  style="border-left: 3px solid var(--color-success)"
                  data-node="roadmap:roadmap-{idx}"
                  onmouseenter={() => hoveredNode = `roadmap:roadmap-${idx}`}
                  onmouseleave={() => hoveredNode = null}
                  onclick={(e) => handleNodeClick(e, `roadmap:roadmap-${idx}`)}
                >
                  <div class="flex items-center gap-2 mb-1">
                    <div class="text-[10px] uppercase tracking-wider font-medium text-[var(--color-success)]">{item.section}</div>
                    <span class="ml-auto text-[10px] font-mono text-[var(--color-success)]">✓</span>
                  </div>
                  <div class="text-xs leading-relaxed text-[var(--color-text-muted)] line-through">{item.text}</div>
                  {#if item.linked_features.length > 0}
                    <div class="mt-2 flex flex-wrap gap-1">
                      {#each item.linked_features as feat}
                        {@const fc = featColor(feat)}
                        <span class="text-[10px] font-mono px-1.5 py-0.5 rounded-[var(--radius-sm)]" style="background: {fc.bg}; color: {fc.text}">{feat}</span>
                      {/each}
                    </div>
                  {/if}
                </div>
              {/each}
            {/if}
          {/if}
        </div>
      </div>

      <!-- Features column (P1: smart fold) -->
      <div class="min-w-0">
        <div class="text-[11px] font-semibold uppercase tracking-widest text-[var(--color-text-muted)] mb-3 px-1">
          Features <span class="text-[var(--color-text-muted)]/60">({activeFeatures.length} active)</span>
        </div>
        <div class="space-y-2">
          {#each visibleFeatures as feat}
            <div
              class="rounded-[var(--radius-md)] border border-[var(--color-border)] bg-[var(--color-bg-elevated)] px-3 py-2.5 text-sm cursor-pointer hover:border-[var(--color-border-strong)] {nodeClass('feature', feat.id)}"
              data-node="feature:{feat.id}"
              onmouseenter={() => hoveredNode = `feature:${feat.id}`}
              onmouseleave={() => hoveredNode = null}
              onclick={(e) => handleNodeClick(e, `feature:${feat.id}`)}
            >
              <div class="flex items-center gap-2 mb-1">
                <span class="w-1.5 h-1.5 rounded-full shrink-0" style="background: {statusColor(feat.status)}"></span>
                <span class="text-[10px] font-mono text-[var(--color-text-muted)]">{feat.id}</span>
                <span class="text-[10px] text-[var(--color-text-muted)] ml-auto">{feat.status}</span>
              </div>
              <div class="text-xs text-[var(--color-text-secondary)] leading-snug">{feat.title}</div>
            </div>
          {:else}
            <div class="text-xs italic text-[var(--color-text-muted)] px-1">No active features</div>
          {/each}

          <!-- P1: Fold toggle -->
          {#if terminalFeatures.length > 0}
            <button
              class="w-full text-[10px] text-[var(--color-text-muted)] hover:text-[var(--color-text-secondary)] py-1.5 px-2 rounded-[var(--radius-sm)] hover:bg-[var(--color-surface-hover)] transition-colors text-left"
              onclick={() => { showAllFeatures = !showAllFeatures; }}
            >
              {showAllFeatures ? '▾ Hide' : '▸ Show'} {terminalFeatures.length} implemented
            </button>
          {/if}
        </div>
      </div>

      <!-- ADR column (reverse order) -->
      <div class="min-w-0">
        <div class="text-[11px] font-semibold uppercase tracking-widest text-[var(--color-text-muted)] mb-3 px-1">
          ADR <span class="text-[var(--color-text-muted)]/60">({overview.adrs.length})</span>
        </div>
        <div class="space-y-2">
          {#each [...overview.adrs].reverse() as adr}
            <div
              class="rounded-[var(--radius-md)] border border-[var(--color-border)] bg-[var(--color-bg-elevated)] px-3 py-2.5 text-sm cursor-pointer hover:border-[var(--color-border-strong)] {nodeClass('adr', adr.id)}"
              data-node="adr:{adr.id}"
              onmouseenter={() => hoveredNode = `adr:${adr.id}`}
              onmouseleave={() => hoveredNode = null}
              onclick={(e) => handleNodeClick(e, `adr:${adr.id}`)}
            >
              <div class="flex items-center gap-2 mb-1">
                <span class="w-1.5 h-1.5 rounded-full shrink-0" style="background: {statusColor(adr.status)}"></span>
                <span class="text-[10px] font-mono text-[var(--color-text-muted)]">{adr.id}</span>
              </div>
              <div class="text-xs text-[var(--color-text-secondary)] leading-snug">{adr.title}</div>
              {#if adr.related_features.length > 0}
                <div class="mt-2 flex flex-wrap gap-1">
                  {#each adr.related_features as feat}
                    {@const fc = featColor(feat)}
                    <span class="text-[10px] font-mono px-1.5 py-0.5 rounded-[var(--radius-sm)]" style="background: {fc.bg}; color: {fc.text}">{feat}</span>
                  {/each}
                </div>
              {/if}
            </div>
          {:else}
            <div class="text-xs italic text-[var(--color-text-muted)] px-1">No ADRs</div>
          {/each}
        </div>
      </div>

      <!-- TODO column -->
      <div class="min-w-0">
        <div class="text-[11px] font-semibold uppercase tracking-widest text-[var(--color-text-muted)] mb-3 px-1">
          TODO <span class="text-[var(--color-text-muted)]/60">({overview.todos.filter(t => !t.done).length})</span>
        </div>

        <!-- Add TODO (P3: data-todo-input for / shortcut) -->
        <form class="flex gap-1.5 mb-3" onsubmit={(e) => { e.preventDefault(); handleAddTodo(); }}>
          <input
            type="text"
            bind:value={newTodoText}
            bind:this={todoInputEl}
            data-todo-input
            placeholder="Add a todo..."
            disabled={adding}
            class="flex-1 rounded-[var(--radius-md)] px-3 py-1.5 text-xs bg-[var(--color-bg)] border border-[var(--color-border)] text-[var(--color-text)] outline-none focus:border-[var(--color-primary)] placeholder:text-[var(--color-text-muted)] transition-colors"
          />
          <button
            type="submit"
            disabled={adding || !newTodoText.trim()}
            class="rounded-[var(--radius-md)] px-2.5 py-1.5 text-xs font-medium bg-[var(--color-primary)] text-white disabled:opacity-30 hover:bg-[var(--color-primary-hover)] transition-colors"
          >+</button>
        </form>

        <!-- Pending TODOs (reverse order) -->
        <div class="space-y-1.5">
          {#each overview.todos.filter(t => !t.done).reverse() as todo}
            <div
              class="rounded-[var(--radius-md)] border border-[var(--color-border)] bg-[var(--color-bg-elevated)] px-3 py-2 text-sm group cursor-pointer hover:border-[var(--color-border-strong)] {nodeClass('todo', `#${todo.id}`)}"
              data-node="todo:#{todo.id}"
              onmouseenter={() => hoveredNode = `todo:#${todo.id}`}
              onmouseleave={() => hoveredNode = null}
              onclick={(e) => handleNodeClick(e, `todo:#${todo.id}`)}
            >
              <div class="flex items-start gap-2">
                <button
                  class="mt-0.5 w-3.5 h-3.5 rounded border border-[var(--color-border-strong)] shrink-0 hover:border-[var(--color-primary)] transition-colors"
                  onclick={(e) => { e.stopPropagation(); handleToggleTodo(todo); }}
                ></button>
                <span class="text-xs text-[var(--color-text-secondary)] leading-relaxed flex-1">
                  {stripAdrTags(todo.content)}
                </span>
                <button
                  class="opacity-0 group-hover:opacity-100 text-[10px] text-[var(--color-text-muted)] hover:text-[var(--color-danger)] transition-all px-1"
                  onclick={(e) => { e.stopPropagation(); handleDeleteTodo(todo); }}
                >✕</button>
              </div>
              {#if todo.adr_refs.length > 0}
                <div class="mt-1.5 flex flex-wrap gap-1 ml-[22px]">
                  {#each todo.adr_refs as ref}
                    <span class="text-[10px] font-mono px-1.5 py-0.5 rounded-[var(--radius-sm)] bg-[var(--color-warning)]/10 text-[var(--color-warning)]">{ref}</span>
                  {/each}
                </div>
              {/if}
            </div>
          {:else}
            <div class="text-xs italic text-[var(--color-text-muted)] px-1">No TODOs</div>
          {/each}
        </div>

        <!-- Done TODOs -->
        {#if overview.todos.filter(t => t.done).length > 0}
          <div class="mt-4 pt-3 border-t border-[var(--color-border)]">
            <div class="text-[10px] font-medium uppercase tracking-wider text-[var(--color-text-muted)] mb-2 px-1">
              Completed ({overview.todos.filter(t => t.done).length})
            </div>
            <div class="space-y-1">
              {#each overview.todos.filter(t => t.done) as todo}
                <div
                  class="rounded-[var(--radius-md)] border border-[var(--color-border)] bg-[var(--color-bg-elevated)] px-3 py-1.5 text-sm group cursor-pointer opacity-50 hover:opacity-70 {nodeClass('todo', `#${todo.id}`)}"
                  data-node="todo:#{todo.id}"
                  onmouseenter={() => hoveredNode = `todo:#${todo.id}`}
                  onmouseleave={() => hoveredNode = null}
                  onclick={(e) => handleNodeClick(e, `todo:#${todo.id}`)}
                >
                  <div class="flex items-start gap-2">
                    <button
                      class="mt-0.5 w-3.5 h-3.5 rounded border border-[var(--color-success)] bg-[var(--color-success)] shrink-0 flex items-center justify-center"
                      onclick={(e) => { e.stopPropagation(); handleToggleTodo(todo); }}
                    >
                      <svg class="w-2 h-2 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="3">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/>
                      </svg>
                    </button>
                    <span class="text-xs text-[var(--color-text-muted)] line-through flex-1">
                      {stripAdrTags(todo.content)}
                    </span>
                    <button
                      class="opacity-0 group-hover:opacity-100 text-[10px] text-[var(--color-text-muted)] hover:text-[var(--color-danger)] transition-all px-1"
                      onclick={(e) => { e.stopPropagation(); handleDeleteTodo(todo); }}
                    >✕</button>
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>
