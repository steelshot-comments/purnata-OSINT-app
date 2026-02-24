<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { ArrowLeft, Plus } from "lucide-svelte";
  import { graph } from "$lib/graph/graph.svelte";
  import NodeCard from "$lib/components/NodeCard.svelte";
  import { fade } from "svelte/transition";

  // Configuration for default properties
  const labelTemplates: Record<string, Record<string, string>> = {
    Email: { address: "", provider: "" },
    Person: { name: "", handle: "" },
    Project: { title: "", status: "active" },
    Server: { ip: "", os: "" },
  };

  // Initialize with one permanent node
  let nodeQueue = $state([
    {
      label: "Email",
      properties: { address: "", provider: "" },
      isNew: false, // The permanent first node starts closed/standard
    },
  ]);

  function addNewStagedNode() {
    nodeQueue.push({
      label: "Email",
      properties: { address: "", provider: "" },
      isNew: true, // New nodes will trigger 'startInEdit'
    });
  }

  async function submitAll() {
    try {
      for (const node of nodeQueue) {
        await invoke("add_node_to_graph", node);
      }
      history.back();
    } catch (error) {
      alert("Error adding nodes: " + error);
    }
  }
</script>

<div class="min-h-screen bg-[#12181b] text-slate-200 flex flex-col font-sans">
  <header
    class="h-16 flex items-center px-4 border-b border-white/5 bg-[#12181b] sticky top-0 z-10"
  >
    <button
      onclick={() => history.back()}
      class="p-2 hover:bg-white/5 rounded-full transition-colors"
    >
      <ArrowLeft size={24} />
    </button>
    <h1 class="ml-4 text-xl font-medium">Create Nodes</h1>
  </header>

  <main class="grow p-6 max-w-2xl w-full mx-auto space-y-6" in:fade>
    <div class="flex items-center justify-between mb-2">
      <h2 class="text-slate-400 text-sm font-medium uppercase tracking-wider">
        Node List
      </h2>
      <button
        onclick={addNewStagedNode}
        class="text-teal-500 hover:text-teal-400 flex items-center gap-1 text-sm font-semibold transition-colors"
      >
        <Plus size={16} /> Add Another Node
      </button>
    </div>

    <div class="space-y-2">
      {#each nodeQueue as node, i}
        <NodeCard
          {node}
          canRemove={nodeQueue.length > 1}
          startInEdit={node.isNew}
          onRemove={() => (nodeQueue = nodeQueue.filter((_, idx) => idx !== i))}
          onUpdate={(updated) => {
            nodeQueue[i] = { ...updated, isNew: false };
          }}
        />
      {/each}
    </div>
  </main>

  <footer class="p-6 bg-[#12181b] border-t border-white/5 flex gap-4">
    <button
      onclick={() => history.back()}
      class="flex-1 py-3 bg-white/5 text-slate-400 font-semibold rounded-xl hover:bg-white/10 transition-all"
    >
      Cancel
    </button>
    <button
      onclick={submitAll}
      class="flex-2 py-3 bg-teal-500/20 text-teal-400 font-bold rounded-xl border border-teal-500/30 hover:bg-teal-500/30 transition-all active:scale-[0.98]"
    >
      Submit to Graph
    </button>
  </footer>
</div>
