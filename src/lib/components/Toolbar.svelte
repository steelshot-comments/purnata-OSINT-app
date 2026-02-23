<script lang="ts">
  import { Toolbar } from "@svar-ui/svelte-toolbar";
  import { WillowDark } from "@svar-ui/svelte-core";
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
    LayoutGrid,
    Network,
    Orbit,
    Pencil,
    CirclePile,
    Shuffle
  } from "lucide-svelte";

  let {
    onToggleView,
    onReset,
    onAddNode,
    onFit,
    onSearch,
    onToggleSelect,
    isSelectMode,
    selectedNode,
    onDelete,
    onChangeLayout,
  } = $props<{
    onToggleView: () => void;
    onReset: () => void;
    onAddNode: () => void;
    onFit: () => void;
    onSearch: () => void;
    onToggleSelect: () => void;
    isSelectMode: boolean;
    selectedNode: any;
    onDelete: () => void;
    onChangeLayout: (name: string) => void;
  }>();

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
      // Dynamically apply a 'selected' class when mode is active
      css: `icon-btn ${isSelectMode ? "select-mode-active" : ""}`,
    },
    {
      id: "fit",
      comp: Focus,
      handler: onFit,
      css: "icon-btn",
    },
    {
      id: "reset",
      comp: RotateCcw,
      handler: onReset,
      css: "icon-btn",
    },
    { comp: "separator" },
    {
      id: "layout-grid",
      comp: LayoutGrid,
      handler: () => onChangeLayout("grid"),
      css: "icon-btn",
      tooltip: "Grid Layout"
    },
    {
      id: "layout-cose",
      comp: Network,
      handler: () => onChangeLayout("cose"),
      css: "icon-btn",
      tooltip: "Force Directed"
    },
    {
      id: "layout-circle",
      comp: Orbit,
      handler: () => onChangeLayout("circle"),
      css: "icon-btn",
      tooltip: "Circle Layout"
    },
    {
      id: "layout-concentric",
      comp: CirclePile,
      handler: () => onChangeLayout("concentric"),
      css: "icon-btn",
      tooltip: "Concentric Layout"
    },
    {
      id: "layout-random",
      comp: Shuffle,
      handler: () => onChangeLayout("random"),
      css: "icon-btn",
      tooltip: "Random Layout"
    },
    { comp: "separator" },
    {
      id: "addNode",
      comp: CirclePlus,
      text: "Add Node",
      css: "add-node-btn",
      handler: onAddNode,
    },
    {
      id: "circlePlus",
      comp: CirclePlus,
      css: "icon-btn",
    },
    {
      id: "edit",
      comp: Pencil,
      css: "icon-btn",
    },
    {
      id: "duplicate",
      comp: CopyPlus,
      css: "icon-btn",
    },
    {
      id: "delete",
      comp: Trash2,
      handler: onDelete,
      css: `icon-btn delete-btn ${!selectedNode ? 'opacity-30 pointer-events-none cursor-not-allowed' : ''}`,
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

  /* Style for items where Lucide is the component */
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
    color: #fbbf24 !important; /* Amber/Yellow icon */
    background: rgba(251, 191, 36, 0.15) !important;
    border: 1px solid rgba(251, 191, 36, 0.3);
  }

  /* Customizing the SVAR built-in button */
  :global(.add-node-btn) {
    background-color: rgba(20, 184, 166, 0.2) !important;
    color: #2dd4bf !important;
    font-weight: 600 !important;
    border-radius: 8px !important;
    border: 1px solid rgba(20, 184, 166, 0.3) !important;
    margin-left: 8px !important;
  }

  .delete-btn {
    /* Layout & Sizing */
    display: flex;
    width: 100%;
    padding-top: 0.5rem; /* py-2 */
    padding-bottom: 0.5rem;
    align-items: center;
    justify-content: center;
    gap: 0.5rem; /* gap-2 */

    /* Colors & Borders */
    background-color: rgba(239, 68, 68, 0.1); /* bg-red-500/10 */
    color: rgb(248, 113, 113); /* text-red-400 */
    border: 1px solid rgba(239, 68, 68, 0.2); /* border-red-500/20 */
    border-radius: 0.25rem; /* rounded */

    /* Typography */
    font-size: 0.75rem; /* text-xs */
    font-weight: 700; /* font-bold */
    transition: background-color 0.2s; /* Smooth hover transition */
  }

  /* Hover State */
  .delete-btn:hover {
    background-color: rgba(239, 68, 68, 0.2); /* hover:bg-red-500/20 */
  }

  :global(.close-btn:hover) {
    color: #f87171 !important;
    background: rgba(248, 113, 113, 0.1) !important;
  }
</style>
