<script lang="ts">
  import { Toolbar } from "@svar-ui/svelte-toolbar";
  import { WillowDark, RadioButtonGroup, RadioButton } from "@svar-ui/svelte-core";

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
    LayoutGrid,
    Network,
    Orbit,
    Pencil,
    CirclePile,
    Shuffle,
    ZoomIn,
    ZoomOut


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
    onDelete: () => void;
    onChangeLayout: (name: string) => void;
    zoomIn: () => void;
    zoomOut: () => void;
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
    // {
    //   id: "layout-grid",
    //   comp: LayoutGrid,
    //   handler: () => onChangeLayout("grid"),
    //   css: "icon-btn",
    //   tooltip: "Grid Layout"
    // },
    // {
    //   id: "layout-cose",
    //   comp: Network,
    //   handler: () => onChangeLayout("cose"),
    //   css: "icon-btn",
    //   tooltip: "Force Directed"
    // },
    // {
    //   id: "layout-circle",
    //   comp: Orbit,
    //   handler: () => onChangeLayout("circle"),
    //   css: "icon-btn",
    //   tooltip: "Circle Layout"
    // },
    // {
    //   id: "layout-concentric",
    //   comp: CirclePile,
    //   handler: () => onChangeLayout("concentric"),
    //   css: "icon-btn",
    //   tooltip: "Concentric Layout"
    // },
    // {
    //   id: "layout-random",
    //   comp: Shuffle,
    //   handler: () => onChangeLayout("random"),
    //   css: "icon-btn",
    //   tooltip: "Random Layout"
    // },
    { comp: "separator" },
    {
      id: "addNode",
      comp: CirclePlus,
      text: "Add Node",
      css: "icon-btn add-node-btn",
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
      css: `icon-btn ${!selectedNode ? 'opacity-30 pointer-events-none' : ''}`,
    },
    {
      id: "duplicate",
      comp: CopyPlus,
      css: `icon-btn ${!selectedNode ? 'opacity-30 pointer-events-none' : ''}`,
    },
    {
      id: "delete",
      comp: Trash2,
      handler: onDelete,
      css: `icon-btn delete-btn ${!selectedNode ? 'opacity-30 pointer-events-none' : ''}`,
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

  :global(.add-node-btn):hover {
    background-color: rgba(20, 184, 166, 0.3) !important;
    color: #2dd4bf !important;
    border: 1px solid rgba(20, 184, 166, 0.4) !important;
  }

  :global(.delete-btn) {
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    border-radius: 0.25rem;
  }

  :global(.delete-btn:hover) {
    background-color: rgba(239, 68, 68, 0.1); /* bg-red-500/10 */
    color: rgb(248, 113, 113); /* text-red-400 */
    border: 1px solid rgba(239, 68, 68, 0.2);
  }

  :global(.layout-radio-pill .svar-radio-group) {
        display: flex;
        background: rgba(255, 255, 255, 0.05);
        border-radius: 99px;
        padding: 2px;
        border: 1px solid rgba(255, 255, 255, 0.1);
    }
    
    :global(.layout-radio-pill .svar-radio-button) {
        border-radius: 99px !important;
        font-size: 11px !important;
        padding: 4px 12px !important;
        border: none !important;
    }

    :global(.layout-radio-pill .svar-radio-button--selected) {
        background: #2dd4bf !important; /* Matches your Teal theme */
        color: #0f171a !important;
    }
</style>
