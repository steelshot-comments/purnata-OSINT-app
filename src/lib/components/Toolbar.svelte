<script lang="ts">
  import { Toolbar } from "@svar-ui/svelte-toolbar";
  import { WillowDark, RadioButtonGroup } from "@svar-ui/svelte-core";

  const layouts = [
    { id: "grid", label: "Grid" },
    { id: "cose", label: "Force Directed" },
    { id: "circle", label: "Circle" },
    { id: "concentric", label: "Concentric" },
    { id: "random", label: "Random" },
  ];

  let selectedLayout = $state("grid");
  
  import {
    Focus,
    Search,
    Table as TableIcon,
    RotateCcw,
    SquareDashedMousePointer,
    ChevronLeft,
    CirclePlus,
    Trash2,
    CopyPlus,
    Pencil,
    ZoomIn,
    ZoomOut
  } from "lucide-svelte";

  // Added selectedCount to the props destructuring and type definition
  let {
    onToggleView,
    onReset,
    onAddNode,
    onFit,
    onSearch,
    onToggleSelect,
    isSelectMode,
    selectedNode,
    selectedCount = 0, // Default to 0
    onDelete,
    onChangeLayout,
    zoomIn,
    zoomOut,
  } = $props<{
    onToggleView: () => void;
    onReset: () => void;
    onAddNode: () => void;
    onFit: () => void;
    onSearch: () => void;
    onToggleSelect: () => void;
    isSelectMode: boolean;
    selectedNode: any;
    selectedCount: number;
    onDelete: () => void;
    onChangeLayout: (name: string) => void;
    zoomIn: () => void;
    zoomOut: () => void;
  }>();

  // Helper function to check if anything is active for bulk actions
  const hasSelection = $derived(selectedCount > 0 || !!selectedNode);

  const items: any[] = $derived([
    {
      id: "back",
      comp: ChevronLeft,
      handler: () => history.back(),
      css: "icon-btn",
    },
    {
      id: "search",
      comp: Search,
      handler: onSearch,
      css: "icon-btn",
    },
    { comp: "separator" },
    {
      id: "toggleView",
      comp: TableIcon,
      handler: onToggleView,
      css: "icon-btn",
    },
    {
      id: "selectMode",
      comp: SquareDashedMousePointer,
      handler: onToggleSelect,
      css: `icon-btn ${isSelectMode ? "select-mode-active" : ""}`,
    },
    {comp: "separator"},
    {
      id: "zoom-in",
      comp: ZoomIn,
      handler: zoomIn,
      css: "icon-btn",
    },
    {
      id: "zoom-out",
      comp: ZoomOut,
      handler: zoomOut,
      css: "icon-btn",
    },
    {
      id: "fit",
      comp: Focus,
      handler: onFit,
      css: "icon-btn",
    },
    {comp: "separator"},
    {
      id: "reset",
      comp: RotateCcw,
      handler: onReset,
      css: "icon-btn",
    },
    { comp: "separator" },
    {
      id: "layout",
      comp: RadioButtonGroup,
      css: "flex items-center gap-2",
      tooltip: "Layout Options",
      props: {
        value: selectedLayout,
        options: layouts,
        onChange: (val: string) => {
          selectedLayout = val;
          onChangeLayout(val);
        },
      },
    },
    { comp: "separator" },
    {
      id: "addNode",
      comp: CirclePlus,
      text: "Add Node",
      css: "icon-btn add-node-btn",
      handler: onAddNode,
    },
    {
      id: "edit",
      comp: Pencil,
      // Enabled only for a single node selection usually, but keeping logic consistent
      css: `icon-btn ${!hasSelection ? 'opacity-30 pointer-events-none' : ''}`,
    },
    {
      id: "duplicate",
      comp: CopyPlus,
      css: `icon-btn ${!hasSelection ? 'opacity-30 pointer-events-none' : ''}`,
    },
    {
      id: "delete",
      comp: Trash2,
      handler: onDelete,
      // Now enables if EITHER a node is in the inspector OR multiple are selected in Cy
      css: `icon-btn delete-btn ${!hasSelection ? 'opacity-30 pointer-events-none' : ''}`,
    },
  ]);
</script>

<WillowDark>
    <Toolbar {items} />
</WillowDark>

<style lang="postcss">
  :global(.svar-toolbar .svar-toolbar-items) {
    width: 100% !important;
  }

  :global(.icon-btn) {
    width: 38px;
    height: 38px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    color: #94a3b8;
    border-radius: 8px;
    transition: all 0.2s ease;
  }

  :global(.icon-btn:hover) {
    color: #f1f5f9;
    background: rgba(255, 255, 255, 0.08);
  }

  :global(.select-mode-active) {
    color: #fbbf24 !important;
    background: rgba(251, 191, 36, 0.15) !important;
    border: 1px solid rgba(251, 191, 36, 0.3);
  }

  :global(.add-node-btn):hover {
    background-color: rgba(20, 184, 166, 0.3) !important;
    color: #2dd4bf !important;
    border: 1px solid rgba(20, 184, 166, 0.4) !important;
  }

  :global(.delete-btn:hover) {
    background-color: rgba(239, 68, 68, 0.1);
    color: rgb(248, 113, 113);
    border: 1px solid rgba(239, 68, 68, 0.2);
  }
</style>