import cytoscape, { type Core } from "cytoscape";

export function createCy(
  container: HTMLElement,
  elements: any,
): Core {
  return cytoscape({
    container,
    elements,
    hideEdgesOnViewport: true,
    hideLabelsOnViewport: true,
    textureOnViewport: true,
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


export const graphState = $state({
    elements: { nodes: [], edges: [] },
    needsRefresh: false
});