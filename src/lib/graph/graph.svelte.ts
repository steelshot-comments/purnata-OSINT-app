import { invoke } from "@tauri-apps/api/core";
import cytoscape, { type Core } from "cytoscape";

export function createCy(container: HTMLElement, elements: any): Core {
  return cytoscape({
    container,
    boxSelectionEnabled: true,
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
        selector: "node:selected",
        style: {
          "border-width": "4px",
          "border-color": "#fbbf24", // Yellow glow for selection
          "background-color": "#14b8a6"
        }
      },
      {
        selector: "edge",
        style: {
          "curve-style": "bezier",
          "target-arrow-shape": "triangle",
          "line-color": "#555",
        },
      },
      {
        selector: "edge:selected",
        style: {
          "line-color": "#fbbf24",
          "width": 2
        }
      }
    ],
    layout: { name: "grid" },
  });
}

export function setSelectionMode(cy: Core | null, isSelectMode: boolean) {
  if (!cy) return;

  if (isSelectMode) {
    // 1. Disable panning so dragging creates a selection box
    cy.userPanningEnabled(false); 
    cy.boxSelectionEnabled(true);
    // Optionally change cursor
    cy.container()!.style.cursor = 'crosshair';
  } else {
    // 2. Re-enable panning for normal navigation
    cy.userPanningEnabled(true);
    cy.container()!.style.cursor = 'default';
  }
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
      // 1. Fetch Graph Data
      const rawGraph: string = await invoke("fetch_graph");
      const graphData = JSON.parse(rawGraph);
      this.nodes = graphData.nodes;
      this.edges = graphData.edges;

      // 2. Fetch Action Map (Unwrap the .message property)
      const rawActions: string = await invoke("get_action_map");
      const actionResponse = JSON.parse(rawActions);

      // This is the fix: assign the inner 'message' object to actionMap
      this.actionMap = actionResponse.message || {};

      console.log("Action Map loaded:", this.actionMap);
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