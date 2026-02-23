<script lang="ts">
  import { graph } from "$lib/graph/graph.svelte";
  import TransformButton from "$lib/components/TransformButton.svelte";
  import { fly } from "svelte/transition";
  import { X, Database, Trash2, Edit, Copy } from "lucide-svelte";
  import { invoke } from "@tauri-apps/api/core";

  // Derive actions automatically whenever selectedNode changes
  let actions = $derived(
    graph.selectedNode ? graph.getActionsForNode(graph.selectedNode.primaryLabel) : []
  );

</script>

{#if graph.selectedNode}
  <aside 
    transition:fly={{ duration: 300 }}
    class="fixed right-0 top-0 h-full w-80 bg-[#0f171a] border-l border-white/10 shadow-2xl z-50 flex flex-col"
  >
    <div class="p-4 border-b border-white/5 flex justify-between items-center bg-white/5">
      <div class="flex items-center gap-2">
        <Database size={16} class="text-teal-400" />
        <span class="text-xs font-bold uppercase tracking-wider text-slate-300">Node Details</span>
      </div>
      <button onclick={() => graph.clearSelection()} class="text-slate-500 hover:text-white">
        <X size={20} />
      </button>
    </div>

    <div class="flex-1 overflow-y-auto p-4 space-y-4">
      <section>
        <p class="text-[10px] text-slate-500 font-bold uppercase mb-2">Properties</p>
        <div class="space-y-2">
          {#each Object.entries(graph.selectedNode.properties || {}) as [key, value]}
            <div class="bg-black/20 p-2 rounded border border-white/5">
              <p class="text-[9px] text-teal-500/70 font-mono uppercase">{key}</p>
              <p class="text-sm text-slate-200 break-all">{value}</p>
            </div>
          {/each}
        </div>
      </section>

      {#if actions.length > 0}
        <section class="pt-4 border-t border-white/5">
          <p class="text-[10px] text-slate-500 font-bold uppercase mb-3">Available Transforms</p>
          <div class="grid gap-2">
            {#each actions as action}
              <TransformButton
                text={action.label} 
                source={action.tool} 
                nodeID={graph.selectedNode.id}
                query={graph.selectedNode.properties[action.queryField]}
              />
            {/each}
          </div>
        </section>
      {/if}
    </div>
  </aside>
{/if}