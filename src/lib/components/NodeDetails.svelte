<script lang="ts">
  import { graph } from "$lib/graph/graph.svelte";
  import TransformButton from "$lib/components/TransformButton.svelte";
  import { fly } from "svelte/transition";
  import { X, Database, ChevronDown, ChevronRight } from "lucide-svelte";

  // Track which accordion item is open. Default to the first one.
  let expandedId = $state<string | null>(null);

  // Auto-expand the first node if the selection changes
  $effect(() => {
    if (graph.selectedNodes.length > 0 && !expandedId) {
      expandedId = graph.selectedNodes[0].id;
    }
  });

  function toggle(id: string) {
    expandedId = expandedId === id ? null : id;
  }
</script>

{#if graph.selectedNodes.length > 0}
  <aside 
    transition:fly={{ x: 300, duration: 300 }}
    class="fixed right-0 top-0 h-full w-80 bg-[#0f171a] border-l border-white/10 shadow-2xl z-50 flex flex-col"
  >
    <div class="p-4 border-b border-white/5 flex justify-between items-center bg-white/5">
      <div class="flex items-center gap-2">
        <Database size={16} class="text-teal-400" />
        <span class="text-xs font-bold uppercase tracking-wider text-slate-300">
            Inspector ({graph.selectedNodes.length})
        </span>
      </div>
      <button onclick={() => graph.clearSelection()} class="text-slate-500 hover:text-white">
        <X size={20} />
      </button>
    </div>

    <div class="flex-1 overflow-y-auto custom-scrollbar">
      {#each graph.selectedNodes as node (node.id)}
        <div class="border-b border-white/5">
          <button 
            onclick={() => toggle(node.id)}
            class="w-full flex items-center gap-3 p-4 hover:bg-white/5 transition-colors text-left"
          >
            {#if expandedId === node.id}
                <ChevronDown size={14} class="text-slate-500" />
            {:else}
                <ChevronRight size={14} class="text-slate-500" />
            {/if}
            <div class="overflow-hidden">
                <p class="text-sm font-medium text-slate-200 truncate">{node.label}</p>
                <p class="text-[10px] text-teal-500/70 font-mono truncate">{node.id}</p>
            </div>
          </button>

          {#if expandedId === node.id}
            <div class="p-4 pt-0 space-y-4 bg-black/10">
              <section>
                <p class="text-[9px] text-slate-500 font-bold uppercase mb-2">Properties</p>
                <div class="space-y-1.5">
                  {#each Object.entries(node.properties || {}) as [key, value]}
                    <div class="bg-black/30 p-2 rounded border border-white/5">
                      <p class="text-[8px] text-teal-500/50 font-mono uppercase leading-tight">{key}</p>
                      <p class="text-xs text-slate-300 break-all">{value}</p>
                    </div>
                  {/each}
                </div>
              </section>

              {#if graph.getActionsForNode(node.primaryLabel).length > 0}
                <section class="pt-3 border-t border-white/5">
                  <p class="text-[9px] text-slate-500 font-bold uppercase mb-2">Transforms</p>
                  <div class="grid gap-2">
                    {#each graph.getActionsForNode(node.primaryLabel) as action}
                      <TransformButton
                        text={action.label} 
                        source={action.tool} 
                        nodeID={node.id}
                        query={node.properties[action.queryField]}
                      />
                    {/each}
                  </div>
                </section>
              {/if}
            </div>
          {/if}
        </div>
      {/each}
    </div>
  </aside>
{/if}

<style>
  .custom-scrollbar::-webkit-scrollbar { width: 4px; }
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.1);
    border-radius: 10px;
  }
</style>