import { invoke } from "@tauri-apps/api/core";
import cytoscape, { type Core } from "cytoscape";

export function createCy(container: HTMLElement, elements: any): Core {
  return cytoscape({
    container,
    elements,
    style: [
      {
        selector: "node",
        style: {
          "background-color": "#2dd4bf",
          label: "data(label)",
          color: "#fff",
          "text-valign": "center",
          "font-size": "10px",
        },
      },
      {
        selector: "edge",
        style: {
          "curve-style": "bezier",
          "target-arrow-shape": "triangle",
          "line-color": "#555",
        },
      },
    ],
    layout: { name: "grid" },
  });
}

class GraphState {
  nodes = $state<any[]>([]);
  edges = $state<any[]>([]);
  actionMap = $state<Record<string, any>>({});
  selectedNode = $state<any>(null);
  isLoading = $state(false);

  // Computed elements for Cytoscape/Table
  elements = $derived({
    nodes: this.nodes.map(n => ({
      group: "nodes",
      data: { 
        id: n.id, 
        label: n.properties.name || n.labels[0] || n.id, 
        properties: n.properties,
        primaryLabel: n.labels[0] // Crucial for ActionMap lookup
      },
      classes: n.labels.join(" ")
    })),
    edges: this.edges.map(e => ({
      group: "edges",
      data: { 
        id: e.id, 
        source: e.source, 
        target: e.target, 
        label: e.rel_type, 
        properties: e.properties
      },
      classes: e.rel_type
    }))
  });

  async loadData() {
    this.isLoading = true;
    try {
      // 1. Fetch Graph
      const rawGraph: string = await invoke("fetch_graph");
      const graphData = JSON.parse(rawGraph);
      this.nodes = graphData.nodes;
      this.edges = graphData.edges;
      
      // 2. Fetch Action Map
      const rawActions: string = await invoke("get_action_map");
      this.actionMap = JSON.parse(rawActions);
    } catch (e) {
      console.error("Failed to load graph data:", e);
    } finally {
      this.isLoading = false;
    }
  }

  getActionsForNode(label: string) {
    return this.actionMap[label] || [];
  }

  selectNode(nodeData: any) {
    this.selectedNode = nodeData;
  }

  clearSelection() {
    this.selectedNode = null;
  }
}

export const graph = new GraphState();