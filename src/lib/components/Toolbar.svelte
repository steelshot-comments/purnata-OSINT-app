<script lang="ts">
    import { Focus, Search, CirclePlus, Table as TableIcon, RotateCcw, SquareDashedMousePointer, Plus, X } from 'lucide-svelte';
    import { goto } from '$app/navigation';

    // Define the interface for the actions
    let { 
        onToggleView, 
        onReset,
        onAddNode,
        onFit, 
        onSearch 
    } = $props<{
        onToggleView?: () => void;
        onReset?: () => void;
        onAddNode?: () => void;
        onFit?: () => void;
        onSearch?: () => void;
    }>();
</script>

<div class="flex items-center gap-2">   
    <button onclick={onSearch} title="Search"><Search size={20}/></button>
    <button onclick={onToggleView} title="Toggle Table/Graph"><TableIcon size={20}/></button>
    <button onclick={onFit} title="Fit Graph"><Focus size={20}/></button>
    <button onclick={() => {}} title="Select"><SquareDashedMousePointer size={20}/></button>
    <button onclick={onReset} title="Reload Data"><RotateCcw size={20}/></button>
    <button onclick={onAddNode} title="Add Node">
        add node
        <!-- <Plus size={20}/> -->
    </button>
    <button onclick={() => {}} title="Circle Plus"><CirclePlus size={20}/></button>
    <button onclick={() => history.back()} title="Close"><X size={20}/></button>
</div>

<!-- <style lang="postcss">
    @layer base{
        button {
            @apply p-2 hover:bg-white/10 rounded-lg transition-colors text-white;
        }
    }
</style> -->