<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Toolbar from "$lib/components/Toolbar.svelte";
  import { onMount, onDestroy, tick } from "svelte";
  import type { Core } from "cytoscape";
  import Table from "$lib/components/Table.svelte";
  import { goto } from "$app/navigation";
  import { graph, createCy, setSelectionMode } from "$lib/graph/graph.svelte";
  import NodeDetails from "$lib/components/NodeDetails.svelte";

  let isLoading = $state(true);
  let viewMode = $state<"graph" | "table">("graph");
  let selectedCount = $state(0);

  async function handleDelete() {
    const targetNodes = graph.selectedNodes;
    if (targetNodes.length === 0) return;

    const msg = targetNodes.length === 1
        ? `Delete ${targetNodes[0].label || "this node"}?`
        : `Delete ${targetNodes.length} selected nodes?`;

    if (!confirm(msg)) return;

    try {
      isLoading = true;
      const idsToDelete = targetNodes.map((n) => n.id);
      await invoke("delete_nodes_bulk", { ids: idsToDelete });
      graph.clearSelection();
      selectedCount = 0;
      await graph.loadData();
      await fetchGraphData(); 
    } catch (e) {
      console.error("Bulk delete failed:", e);
    } finally {
      isLoading = false;
    }
  }

  let elements = $state<{ nodes: any[]; edges: any[] }>({ nodes: [], edges: [] });
  let container = $state<HTMLDivElement | null>(null);
  let isSelectMode = $state(false);
  let cy: Core | null = null;

  function closeInspector() {
    graph.clearSelection();
    cy?.$(":selected").unselect();
  }

  function toggleView() {
    viewMode = viewMode === "graph" ? "table" : "graph";
    if (viewMode === "table") closeInspector();
  }

  function toggleSelectMode() {
    isSelectMode = !isSelectMode;
    setSelectionMode(cy, isSelectMode);
  }

  function updateSelectionCount() {
    if (!cy) return;
    const selected = cy.$(":selected");
    selectedCount = selected.length;
    // Map all selected elements to the global graph state
    graph.selectedNodes = selected.map(node => node.data());
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
            properties: node.properties || {},
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
    if (!container || !graph.elements.nodes.length) return;
    if (cy) { cy.destroy(); cy = null; }

    tick().then(() => {
      cy = createCy(container!, graph.elements);
      cy.on("select unselect", () => updateSelectionCount());
      cy.on("tap", (evt) => {
        if (evt.target === cy) {
          graph.clearSelection();
          updateSelectionCount();
        }
      });
    });
  });

  onMount(() => {
    graph.loadData();
    fetchGraphData();
  });

  onDestroy(() => { if (cy) cy.destroy(); });
</script>

<div class="w-screen h-screen relative flex flex-col bg-[#0f171a] overflow-hidden">
  <div class="h-16 bg-[#1a2a26] border-b border-white/10 shrink-0 z-30">
    <Toolbar
      isSelectMode={isSelectMode}
      selectedCount={selectedCount}
      selectedNode={graph.selectedNode}
      onToggleView={toggleView}
      onToggleSelect={toggleSelectMode}
      onDelete={handleDelete}
      onReset={() => fetchGraphData()}
      onFit={() => cy?.fit()}
      onChangeLayout={(n) => cy?.layout({name: n}).run()}
      onAddNode={() => goto("add-node")}
      onSearch={() => goto("search")}
      zoomIn={() => cy?.zoom(cy.zoom() * 1.2)}
      zoomOut={() => cy?.zoom(cy.zoom() * 0.8)}
    />
  </div>

  <div class="relative grow w-full h-full flex overflow-hidden">
    <div
      bind:this={container}
      class="w-full h-full bg-[#0c1113]"
      class:hidden={viewMode !== "graph"}
      class:opacity-50={isLoading}
    ></div>

    {#if viewMode === "table"}
      <div class="absolute inset-0 z-10 bg-[#0f171a]">
        <Table bind:data={elements} />
      </div>
    {/if}

    {#if isLoading}
      <div class="absolute inset-0 z-50 flex flex-col items-center justify-center backdrop-blur-sm bg-black/20">
        <div class="w-12 h-12 border-4 border-teal-500/20 border-t-teal-500 rounded-full animate-spin"></div>
      </div>
    {/if}

    <NodeDetails />
  </div>
</div>