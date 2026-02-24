<script lang="ts">
  import {
    ChevronDown,
    Database,
    GitBranch,
    Search,
    Info,
  } from "lucide-svelte";
  let { data = $bindable() } = $props<{
    data: { nodes: any[]; edges: any[] };
  }>();

  console.log(data);

  // State to track which sections are open
  let openSections = $state({ nodes: true, edges: false });

  // Helper to format property values
  function formatValue(val: any): string {
    if (typeof val === "object") return JSON.stringify(val);
    return String(val);
  }
</script>

<div
  class="w-full h-full flex flex-col border border-white/10 rounded-xl overflow-hidden bg-slate-900 text-sm"
>
  <button
    onclick={() => (openSections.nodes = !openSections.nodes)}
    class="w-full flex items-center justify-between p-4 bg-slate-800 hover:bg-slate-700 transition-colors border-b border-white/5"
  >
    <div class="flex items-center gap-2">
      <span class="text-teal-400">●</span>
      <span class="font-bold text-white text-lg">Nodes</span>
      <span class="bg-slate-900 px-2 py-0.5 rounded text-xs text-slate-400"
        >{data.nodes.length}</span
      >
    </div>
    <span
      class="transform transition-transform {openSections.nodes
        ? 'rotate-180'
        : ''}">
        <ChevronDown size={16} class="text-slate-400" />
        </span
    >
  </button>

  {#if openSections.nodes}
    <div class="overflow-x-auto max-h-[60vh] custom-scrollbar">
      <table class="w-full text-left border-collapse">
        <thead>
          <tr
            class="bg-slate-900 text-slate-400 uppercase text-[10px] tracking-wider"
          >
            <th class="p-3 border-b border-white/5">ID</th>
            <th class="p-3 border-b border-white/5">Labels</th>
            <th class="p-3 border-b border-white/5">Properties</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-white/5">
          {#each data.nodes as node}
            <tr class="hover:bg-white/5 transition-colors">
              <td class="p-3 font-mono text-teal-300 text-[10px] break-all">
                {node.data.id}
              </td>

              <td class="p-3">
                <span
                  class="bg-teal-500/20 text-teal-300 px-2 py-0.5 rounded-full text-[10px]"
                >
                  {node.data.label || node.classes}
                </span>
              </td>

              <td class="p-3">
                <div class="grid grid-cols-1 gap-1">
                  {#each Object.entries(node.data.properties || {}) as [key, val]}
                    <div class="flex gap-2 text-[11px]">
                      <span class="text-slate-500 font-bold lowercase"
                        >{key}:</span
                      >
                      <span class="text-slate-200">{formatValue(val)}</span>
                    </div>
                  {:else}
                    <span class="text-slate-600 italic">No properties</span>
                  {/each}
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}

  <button
    onclick={() => (openSections.edges = !openSections.edges)}
    class="w-full flex items-center justify-between p-4 bg-slate-800 hover:bg-slate-700 transition-colors border-t border-white/10"
  >
    <div class="flex items-center gap-2">
      <span class="text-amber-400">➔</span>
      <span class="font-bold text-white text-lg">Edges (Relationships)</span>
      <span class="bg-slate-900 px-2 py-0.5 rounded text-xs text-slate-400"
        >{data.edges.length}</span
      >
    </div>
    <span
      class="transform transition-transform {openSections.edges
        ? 'rotate-180'
        : ''}">
        <ChevronDown size={16} class="text-slate-400" />
        
        </span
    >
  </button>

  {#if openSections.edges}
    <div class="overflow-x-auto max-h-[40vh] custom-scrollbar">
      <table class="w-full text-left border-collapse">
        <thead>
          <tr
            class="bg-slate-800/50 text-slate-400 uppercase text-[10px] tracking-wider"
          >
            <th class="p-3 border-b border-white/5">Source</th>
            <th class="p-3 border-b border-white/5">Type</th>
            <th class="p-3 border-b border-white/5">Target</th>
            <th class="p-3 border-b border-white/5">Properties</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-white/5">
          {#each data.edges as edge}
            <tr class="hover:bg-white/5 transition-colors">
              <td class="p-3 font-mono text-slate-400">{edge.source}</td>
              <td class="p-3">
                <span
                  class="bg-amber-500/20 text-amber-300 px-2 py-0.5 rounded text-[10px] font-bold"
                >
                  {edge.rel_type}
                </span>
              </td>
              <td class="p-3 font-mono text-slate-400">{edge.target}</td>
              <td class="p-3 text-xs text-slate-400">
                {Object.keys(edge.properties).length > 0
                  ? JSON.stringify(edge.properties)
                  : "—"}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>

<style>
  thead th {
    position: sticky;
    top: 0;
    z-index: 10;
  }

  /* Thin scrollbar for a cleaner look */
  .custom-scrollbar::-webkit-scrollbar {
    width: 6px;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.1);
    border-radius: 10px;
  }
</style>