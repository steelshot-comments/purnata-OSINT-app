<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Toolbar from "$lib/components/Toolbar.svelte";
  import { onMount, onDestroy, tick } from "svelte";
  import cytoscape from "cytoscape";
  import type { Core } from "cytoscape";
  import Table from "$lib/components/Table.svelte";
  import { goto } from "$app/navigation";
  import { graphState, createCy } from "$lib/graph/graph.svelte";

  // Svelte 5 Runes
  let isLoading = $state(true);
  let viewMode = $state<"graph" | "table">("graph");
  let elements = $state<{ nodes: any[]; edges: any[] }>({
    nodes: [],
    edges: [],
  });
  let container = $state<HTMLDivElement | null>(null);
  let cy: Core | null = null;

  function toggleView() {
    viewMode = viewMode === "graph" ? "table" : "graph";
  }

  // 2. Logic for Graph Operations
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
      console.log(data);
      const graphData = JSON.parse(data);

      const newElements = { nodes: [], edges: [] };

      // Transform Nodes
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

      // Transform Edges
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
      console.error("Error:", error);
    } finally {
      isLoading = false;
    }
  }

  // Svelte 5 Effect: Runs when 'elements' or 'container' changes
  $effect(() => {
    if (!container || cy) return;
    if (
      (elements.nodes.length > 0 || elements.edges.length > 0) &&
      container &&
      !cy
    ) {
      const setup = async () => {
        await tick();
        if (!container || cy) return;
        console.log("Initializing Cytoscape with elements:", elements);
        cy = createCy(container, elements);

        (window as any).cy = cy;

        cy.on("tap", "node", function (evt) {
          const node = evt.target;
          console.log("Tapped node:", node.data());
          alert(
            `Node: ${node.data().label}\nProperties: ${JSON.stringify(
              node.data().properties,
              null,
              2,
            )}`,
          );
        });

        cy.on("tap", "edge", function (evt) {
          const edge = evt.target;
          console.log("Tapped edge:", edge.data());
          alert(
            `Edge: ${edge.data().label}\nSource: ${edge.data().source}\nTarget: ${edge.data().target}\nProperties: ${JSON.stringify(
              edge.data().properties,
              null,
              2,
            )}`,
          );
        });

        cy.add({
          group: "nodes",
          data: { id: "manual_test", label: "TEST NODE" },
          position: { x: 750, y: 200 }, // Middle of your 1536 container
        });

        // cy.on("render", () => {
        //   console.log("Cytoscape just rendered a frame!");
        // });

        cy.ready(() => {
          cy?.fit();
          cy?.center();
        });
      };

      setup();
    }
    return () => {
      if (cy) {
        console.log("Destroying CY instance");
        cy.destroy();
        cy = null;
      }
    };
  });

  onMount(async () => {
    // if (graphState.needsRefresh) {
    await fetchGraphData();
    graphState.needsRefresh = false;
    // }
  });

  onDestroy(() => {
    if (cy) cy.destroy();
  });
</script>

<div class="w-screen h-screen relative flex flex-col">
  <div
    class="h-16 bg-[#1a2a26] flex items-center px-4 gap-4 border-b border-white/10 shrink-0"
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

  <div class="relative grow overflow-hidden w-full h-full">
    {#if isLoading}
      <div
        class="absolute inset-0 z-50 flex flex-col items-center justify-center backdrop-blur-sm"
      >
        <div
          class="w-12 h-12 border-4 border-teal-500/20 border-t-teal-500 rounded-full animate-spin"
        ></div>
        <p class="mt-4 text-teal-500 font-medium animate-pulse">
          Loading Graph Data...
        </p>
      </div>
    {/if}

    <div
      bind:this={container}
      id="container"
      class="w-full h-full block relative"
      class:none={viewMode == "graph"}
    ></div>
    {#if viewMode === "table"}
      <Table bind:data={elements} />  
    {/if}
  </div>
</div>

<style>
</style>
