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
          "border-color": "#fbbf24", 
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
    cy.userPanningEnabled(false); 
    cy.boxSelectionEnabled(true);
    cy.container()!.style.cursor = 'crosshair';
  } else {
    cy.userPanningEnabled(true);
    cy.container()!.style.cursor = 'default';
  }
}

class GraphState {
  nodes = $state<any[]>([]);
  edges = $state<any[]>([]);
  actionMap = $state<Record<string, any>>({});
  // Updated to handle multiple selections
  selectedNodes = $state<any[]>([]);
  isLoading = $state(false);

  // Helper for single-node legacy components
  selectedNode = $derived(this.selectedNodes.length > 0 ? this.selectedNodes[0] : null);

  elements = $derived({
    nodes: this.nodes.map(n => ({
      group: "nodes",
      data: {
        id: n.id,
        label: n.properties.name || n.labels[0] || n.id,
        properties: n.properties,
        primaryLabel: n.labels[0] 
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
      const rawGraph: string = await invoke("fetch_graph");
      const graphData = JSON.parse(rawGraph);
      this.nodes = graphData.nodes;
      this.edges = graphData.edges;

      const rawActions: string = await invoke("get_action_map");
      const actionResponse = JSON.parse(rawActions);
      this.actionMap = actionResponse.message || {};
    } catch (e) {
      console.error("Failed to load graph data:", e);
    } finally {
      this.isLoading = false;
    }
  }

  getActionsForNode(label: string) {
    return this.actionMap[label] || [];
  }

  clearSelection() {
    this.selectedNodes = [];
  }
}

export const graph = new GraphState();