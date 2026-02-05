<script lang="ts">
    import { Toolbar } from "@svar-ui/svelte-toolbar";
    import { WillowDark } from "@svar-ui/svelte-core";
    import { 
        Focus, Search, Table as TableIcon, 
        RotateCcw, SquareDashedMousePointer, X, CirclePlus 
    } from 'lucide-svelte';

    let { onToggleView, onReset, onAddNode, onFit, onSearch } = $props<{
        onToggleView: () => void;
        onReset: () => void;
        onAddNode: () => void;
        onFit: () => void;
        onSearch: () => void;
    }>();

    // Casting to 'any' here is the "escape hatch" to allow Lucide components
    // without triggering the 'string is not assignable to IToolbarItem' error.
    const items: any[] = [
        { 
            id: "search", 
            comp: Search, 
            handler: onSearch,
            css: "icon-btn" 
        },
        { comp: "separator" },
        { 
            id: "toggleView", 
            comp: TableIcon, 
            handler: onToggleView,
            css: "icon-btn"
        },
        { 
            id: "fit", 
            comp: Focus, 
            handler: onFit,
            css: "icon-btn"
        },
        { 
            id: "reset", 
            comp: RotateCcw, 
            handler: onReset,
            css: "icon-btn"
        },
        { comp: "spacer" }, 
        { 
            id: "addNode", 
            comp: CirclePlus, 
            text: "Add Node", 
            // We use 'css' to inject the Plus icon via a background or pseudo-element 
            // if SVAR types won't let us pass the Lucide component to the 'icon' field.
            css: "add-node-btn",
            handler: onAddNode
        },
        { 
            id: "circlePlus", 
            comp: CirclePlus,
            css: "icon-btn"
        },
        { comp: "separator" },
        { 
            id: "close", 
            comp: X, 
            handler: () => history.back(),
            css: "icon-btn close-btn" 
        },
    ];
</script>

<WillowDark>
    <div class="svar-toolbar-wrapper">
        <Toolbar {items} overflow="menu" />
    </div>
</WillowDark>

<style>
    :global(.svar-toolbar) {
        background-color: #12181b !important;
        border-bottom: 1px solid rgba(255,255,255,0.05);
        height: 50px !important;
        padding: 0 8px !important;
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
        background: rgba(255,255,255,0.08);
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

    :global(.close-btn:hover) {
        color: #f87171 !important;
        background: rgba(248, 113, 113, 0.1) !important;
    }
</style>