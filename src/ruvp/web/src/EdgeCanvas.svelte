<script>
  let { edges, nodePositions, hoveredNode, highlightedIds, highlightTiers } = $props();

  function getEdgePaths() {
    if (!edges || !nodePositions) return [];
    const paths = [];

    for (const edge of edges) {
      const fromKey = `${edge.from_type}:${edge.from_id}`;
      const toKey = `${edge.to_type}:${edge.to_id}`;
      const from = nodePositions[fromKey];
      const to = nodePositions[toKey];
      if (!from || !to) continue;

      const bothInHighlight = hoveredNode && highlightedIds?.has(fromKey) && highlightedIds?.has(toKey);
      // Direct: both ends are in the direct set
      const isDirect = bothInHighlight && highlightTiers?.direct?.has(fromKey) && highlightTiers?.direct?.has(toKey);
      // Indirect: both ends are highlighted but at least one is in the indirect set
      const isIndirect = bothInHighlight && !isDirect;

      let leftNode, rightNode;
      if (from.x < to.x) {
        leftNode = from;
        rightNode = to;
      } else {
        leftNode = to;
        rightNode = from;
      }

      const startX = leftNode.x + leftNode.width / 2;
      const startY = leftNode.y;
      const endX = rightNode.x - rightNode.width / 2;
      const endY = rightNode.y;

      const dx = endX - startX;
      const cpOffset = Math.abs(dx) * 0.4;
      const d = `M ${startX} ${startY} C ${startX + cpOffset} ${startY}, ${endX - cpOffset} ${endY}, ${endX} ${endY}`;

      paths.push({ d, isDirect, isIndirect, fromKey, toKey });
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
      stroke={path.isDirect ? 'var(--color-primary)' : path.isIndirect ? 'var(--color-primary)' : 'var(--color-border)'}
      stroke-width={path.isDirect ? 1.5 : path.isIndirect ? 1 : 0.75}
      stroke-dasharray={path.isDirect ? 'none' : path.isIndirect ? '4 3' : '3 3'}
      opacity={hoveredNode ? (path.isDirect ? 0.9 : path.isIndirect ? 0.45 : 0.1) : 0.3}
      style="transition: all 200ms ease"
    />
    {#if path.isDirect}
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
