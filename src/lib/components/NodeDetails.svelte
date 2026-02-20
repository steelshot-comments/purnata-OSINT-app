<script lang="ts">
  import { graph } from "$lib/graph/graph.svelte";
  import TransformButton from "./TransformButton.svelte";
  import { fly } from "svelte/transition";
  import { X, Database, Trash2, Edit, Copy } from "lucide-svelte";

  // Derive actions automatically whenever selectedNode changes
  let actions = $derived(
    graph.selectedNode ? graph.getActionsForNode(graph.selectedNode.primaryLabel) : []
  );

  async function handleDelete() {
    if (!confirm("Delete this node?")) return;
    // Call your Rust command
    graph.clearSelection();
    await graph.loadData();
  }
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

    <div class="p-4 bg-black/40 border-t border-white/5 space-y-2">
      <div class="flex gap-2">
        <button class="flex-1 py-2 bg-white/5 hover:bg-white/10 rounded text-xs flex items-center justify-center gap-2">
          <Edit size={14} /> Edit
        </button>
        <button class="flex-1 py-2 bg-white/5 hover:bg-white/10 rounded text-xs flex items-center justify-center gap-2">
          <Copy size={14} /> Clone
        </button>
      </div>
      <button 
        onclick={handleDelete}
        class="w-full py-2 bg-red-500/10 hover:bg-red-500/20 text-red-400 border border-red-500/20 rounded text-xs font-bold flex items-center justify-center gap-2"
      >
        <Trash2 size={14} /> Delete Node
      </button>
    </div>
  </aside>
{/if}