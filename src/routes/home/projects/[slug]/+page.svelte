<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Toolbar from "$lib/components/Toolbar.svelte";
  import { onMount, onDestroy, tick } from "svelte";
  import type { Core } from "cytoscape";
  import Table from "$lib/components/Table.svelte";
  import { goto } from "$app/navigation";
  import { graph, createCy } from "$lib/graph/graph.svelte";
  import NodeDetails from "$lib/components/NodeDetails.svelte";
  import TransformButton from "$lib/components/TransformButton.svelte";

  // Svelte 5 Runes
  let isLoading = $state(true);
  let viewMode = $state<"graph" | "table">("graph");
  let selectedNodeData = $state<any>(null);

  async function deleteNode(data: any) {
    const confirmDelete = confirm(`Delete ${data.label}?`);
    if (!confirmDelete) return;

    try {
      await invoke("delete_node_from_graph", {
        idValue: selectedNodeData.id,
      });
      selectedNodeData = null;
      await fetchGraphData();
    } catch (e) {
      console.error(e);
    }
  }

  let elements = $state<{ nodes: any[]; edges: any[] }>({
    nodes: [],
    edges: [],
  });
  let container = $state<HTMLDivElement | null>(null);
  let cy: Core | null = null;

  function updateSelectedNode(data: any) {
    selectedNodeData = data;
  }

  function closeInspector() {
    selectedNodeData = null;
    cy?.$(":selected").unselect();
  }

  function toggleView() {
    viewMode = viewMode === "graph" ? "table" : "graph";
    if (viewMode === "table") closeInspector();
  }

  function resetGraph() {
    fetchGraphData();
  }

  function fitGraph() {
    cy?.fit();
    cy?.center();
  }

  function addNode() {
    goto("/home/projects/addNode");
  }

  function onSearch() {
    return;
  }

  async function fetchGraphData() {
    try {
      isLoading = true;
      const data: string = await invoke("fetch_graph");
      const graphData = JSON.parse(data);

      const newElements = { nodes: [], edges: [] };

      graphData.nodes.forEach((node: any) => {
        newElements.nodes.push({
          group: "nodes",
          data: {
            id: node.id,
            label: node.properties.name || node.labels[0] || node.id,
            properties: node.properties,
          },
          classes: node.labels.join(" "),
        });
      });

      graphData.edges.forEach((edge: any) => {
        newElements.edges.push({
          group: "edges",
          data: {
            id: edge.id,
            source: edge.source,
            target: edge.target,
            label: edge.rel_type,
            properties: edge.properties,
          },
          classes: edge.rel_type,
        });
      });

      elements = newElements;
    } catch (error) {
      console.error("Error fetching graph:", error);
    } finally {
      isLoading = false;
    }
  }

  $effect(() => {
    if (!container) return;
    
    // Cleanup old instance
    if (cy) {
      cy.destroy();
      cy = null;
    }

    if (graph.elements.nodes.length > 0) {
      tick().then(() => {
        cy = createCy(container!, graph.elements);
        
        cy.on("tap", "node", (evt) => {
          graph.selectNode(evt.target.data());
        });

        cy.on("tap", (evt) => {
          if (evt.target === cy) graph.clearSelection();
        });
      });
    }
  });

  onMount(async () => {
    graph.loadData();
  });

  onDestroy(() => {
    if (cy) cy.destroy();
  });
</script>

<div
  class="w-screen h-screen relative flex flex-col bg-[#0f171a] overflow-hidden"
>
  <div
    class="h-16 bg-[#1a2a26] flex items-center px-4 gap-4 border-b border-white/10 shrink-0 z-30"
  >
    <button
      onclick={() => history.back()}
      class="px-4 py-2 bg-white/10 text-white hover:bg-white/20 rounded-lg text-sm transition-colors"
    >
      Back
    </button>
    <Toolbar
      {onSearch}
      onToggleView={toggleView}
      onReset={resetGraph}
      onFit={fitGraph}
      onAddNode={addNode}
    />
  </div>

  <div class="relative grow w-full h-full flex overflow-hidden">
    <aside>
      
    </aside>
    
    <div
      bind:this={container}
      class="w-full h-full bg-[#0c1113] transition-opacity duration-300"
      class:hidden={viewMode !== "graph"}
      class:opacity-50={isLoading}
    ></div>

    {#if viewMode === "table"}
      <div class="absolute inset-0 z-10 bg-[#0f171a]">
        <Table bind:data={elements} />
      </div>
    {/if}

    {#if isLoading}
      <div
        class="absolute inset-0 z-50 flex flex-col items-center justify-center backdrop-blur-sm bg-black/20"
      >
        <div
          class="w-12 h-12 border-4 border-teal-500/20 border-t-teal-500 rounded-full animate-spin"
        ></div>
        <p class="mt-4 text-teal-500 font-medium animate-pulse">
          Loading Graph Data...
        </p>
      </div>
    {/if}

    <NodeDetails />
  </div>
</div>

<style>
  :global(.hidden) {
    display: none !important;
  }

  ::-webkit-scrollbar {
    width: 4px;
  }
  ::-webkit-scrollbar-track {
    background: transparent;
  }
  ::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.1);
    border-radius: 10px;
  }
</style>


      <!-- <p class="text-[10px] font-mono text-slate-500 mt-1 uppercase tracking-tighter">UID: {nodeData.id}</p> -->
