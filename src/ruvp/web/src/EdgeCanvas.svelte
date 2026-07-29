<script>
  let { edges, nodePositions, hoveredNode, highlightedIds } = $props();

  function getEdgePaths() {
    if (!edges || !nodePositions) return [];
    const paths = [];

    for (const edge of edges) {
      const fromKey = `${edge.from_type}:${edge.from_id}`;
      const toKey = `${edge.to_type}:${edge.to_id}`;
      const from = nodePositions[fromKey];
      const to = nodePositions[toKey];
      if (!from || !to) continue;

      const isActive = hoveredNode && highlightedIds?.has(fromKey) && highlightedIds?.has(toKey);

      let leftNode, rightNode;
      if (from.x < to.x) {
        leftNode = from;
        rightNode = to;
      } else {
        leftNode = to;
        rightNode = from;
      }

      // Right-side midpoint of left node → left-side midpoint of right node
      const startX = leftNode.x + leftNode.width / 2;
      const startY = leftNode.y;
      const endX = rightNode.x - rightNode.width / 2;
      const endY = rightNode.y;

      const dx = endX - startX;
      const cpOffset = Math.abs(dx) * 0.4;
      const d = `M ${startX} ${startY} C ${startX + cpOffset} ${startY}, ${endX - cpOffset} ${endY}, ${endX} ${endY}`;

      paths.push({ d, isActive, fromKey, toKey });
    }
    return paths;
  }

  let edgePaths = $derived(getEdgePaths());
</script>

<svg class="absolute inset-0 w-full h-full pointer-events-none z-0" style="overflow: visible">
  {#each edgePaths as path}
    <path
      d={path.d}
      fill="none"
      stroke={path.isActive ? 'var(--color-primary)' : 'var(--color-border)'}
      stroke-width={path.isActive ? 1.5 : 0.75}
      stroke-dasharray={path.isActive ? 'none' : '3 3'}
      opacity={hoveredNode ? (path.isActive ? 0.9 : 0.1) : 0.3}
      style="transition: all 200ms ease"
    />
    {#if path.isActive}
      <circle
        cx={path.d.split(' ')[1]}
        cy={path.d.split(' ')[2]}
        r="3"
        fill="var(--color-primary)"
        opacity="0.8"
      />
    {/if}
  {/each}
</svg>
