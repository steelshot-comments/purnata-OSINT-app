<script lang="ts">
    import { Trash2, Tag, Pen, Check, X, Plus } from "lucide-svelte";
    import { slide } from "svelte/transition";

    let { node, onRemove, onUpdate, canRemove = true, startInEdit = false } = $props<{
        node: { label: string; properties: Record<string, string> };
        onRemove: () => void;
        onUpdate: (updatedNode: any) => void;
        canRemove?: boolean;
        startInEdit?: boolean;
    }>();

    const availableLabels = ["Email", "Person", "Project", "Server", "Domain"];
    
    const labelTemplates: Record<string, Record<string, string>> = {
        "Email": { "address": "", "provider": "" },
        "Person": { "name": "", "handle": "" },
        "Project": { "title": "", "status": "active" },
        "Server": { "ip": "", "os": "" },
        "Domain": { "url": "", "registrar": "" }
    };

    let isExpanded = $state(startInEdit);
    let isEditing = $state(startInEdit);
    let editLabel = $state(node.label);
    let editableProps = $state<{ key: string; value: string }[]>([]);

    // Initialize props if starting in edit mode
    if (startInEdit) {
        editableProps = Object.entries(node.properties).map(([key, value]) => ({ key, value: value as string }));
    }

    function startEditing() {
        editLabel = node.label;
        editableProps = Object.entries(node.properties).map(([key, value]) => ({
            key,
            value: value as string,
        }));
        isEditing = true;
        isExpanded = true;
    }

    // This ensures changing the dropdown updates the fields
    function handleLabelChange(newLabel: string) {
        editLabel = newLabel;
        const template = labelTemplates[newLabel];
        if (template) {
            editableProps = Object.entries(template).map(([key, value]) => ({
                key,
                value: value as string
            }));
        }
    }

    function saveEdit() {
        const updatedProperties = editableProps.reduce((acc, prop) => {
            if (prop.key) acc[prop.key] = prop.value;
            return acc;
        }, {} as Record<string, string>);

        onUpdate({ label: editLabel, properties: updatedProperties });
        isEditing = false;
    }

    function handleAction(e: MouseEvent, action: () => void) {
        e.stopPropagation();
        action();
    }
</script>

<div class="bg-[#2a2e33]/30 border border-white/10 rounded-xl overflow-hidden mb-3 transition-all {isEditing ? 'border-teal-500/50 ring-1 ring-teal-500/20' : 'hover:border-white/20'}">
    <div class="p-4 flex items-center justify-between cursor-pointer select-none" onclick={() => (isExpanded = !isExpanded)}>
        <div class="flex items-center gap-3">
            <div class="p-2 {isEditing ? 'bg-teal-500/20 text-teal-400' : 'bg-white/5 text-slate-400'} rounded-lg transition-colors">
                <Tag size={18} />
            </div>
            <div>
                <h3 class="font-medium text-slate-200">{isEditing ? editLabel : node.label}</h3>
                <p class="text-xs text-slate-500">{isEditing ? editableProps.length : Object.keys(node.properties).length} properties</p>
            </div>
        </div>

        <div class="flex items-center gap-1">
            {#if !isEditing}
                <button onclick={(e) => handleAction(e, startEditing)} class="p-2 text-slate-400 hover:text-teal-400 hover:bg-white/5 rounded-lg transition-colors">
                    <Pen size={18} />
                </button>
            {:else}
                <button onclick={(e) => handleAction(e, saveEdit)} class="p-2 text-teal-400 hover:bg-teal-400/10 rounded-lg"><Check size={20} /></button>
                <button onclick={(e) => handleAction(e, () => (isEditing = false))} class="p-2 text-red-400 hover:bg-red-400/10 rounded-lg"><X size={20} /></button>
            {/if}
            
            {#if canRemove}
                <button onclick={(e) => handleAction(e, onRemove)} class="p-2 text-slate-400 hover:text-red-400 hover:bg-red-400/10 rounded-lg transition-colors">
                    <Trash2 size={18} />
                </button>
            {/if}
        </div>
    </div>

    {#if isExpanded}
        <div transition:slide class="px-4 pb-4 border-t border-white/5 pt-3 space-y-3">
            {#if isEditing}
                <div class="space-y-1">
                    <label class="text-[10px] uppercase text-slate-500 font-bold ml-1">Node Type</label>
                    <select 
                        bind:value={editLabel} 
                        onchange={(e) => handleLabelChange((e.target as HTMLSelectElement).value)}
                        class="w-full bg-black/40 border border-white/10 rounded p-2 text-sm text-slate-200 outline-none focus:border-teal-500/50"
                    >
                        {#each availableLabels as label}
                            <option value={label}>{label}</option>
                        {/each}
                    </select>
                </div>

                <div class="space-y-2">
                    <label class="text-[10px] uppercase text-slate-500 font-bold ml-1">Properties</label>
                    {#each editableProps as prop, i}
                        <div class="grid grid-cols-[1fr_1fr_auto] gap-2 items-center">
                            <input bind:value={prop.key} class="bg-black/20 border border-white/10 rounded p-1.5 text-xs text-teal-400 outline-none focus:border-teal-500/50" />
                            <input bind:value={prop.value} class="bg-black/20 border border-white/10 rounded p-1.5 text-xs text-slate-200 outline-none focus:border-teal-500/50" />
                            <button onclick={() => editableProps = editableProps.filter((_, idx) => idx !== i)} class="text-slate-500 hover:text-red-400"><Trash2 size={14} /></button>
                        </div>
                    {/each}
                </div>
                <button onclick={() => editableProps.push({key: "", value: ""})} class="w-full py-2 border border-dashed border-white/10 rounded text-xs text-slate-500 hover:text-teal-400 transition-all">+ Add Property</button>
            {:else}
                {#each Object.entries(node.properties) as [key, value]}
                    <div class="flex justify-between text-sm">
                        <span class="text-slate-500">{key}:</span>
                        <span class="text-slate-300 font-mono">{value}</span>
                    </div>
                {/each}
            {/if}
        </div>
    {/if}
</div>