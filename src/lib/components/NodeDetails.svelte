<script lang="ts">
  import { Check, Fingerprint, Database, Copy, X, Trash2 } from "lucide-svelte";
  import { slide } from "svelte/transition";

  // Props using Svelte 5 runes
  let { nodeData, onClose, onDelete } = $props<{
    nodeData: any;
    onClose: () => void;
    onDelete: (data: any) => Promise<void>;
  }>();

  let copiedKey = $state<string | null>(null);

  function copyToClipboard(value: string, key: string) {
    navigator.clipboard.writeText(value);
    copiedKey = key;
    setTimeout(() => (copiedKey = null), 2000);
  }
</script>

{#if nodeData}
  <aside 
    transition:slide={{ axis: 'x', duration: 250 }}
    class="absolute right-0 top-0 h-full w-80 border-l border-white/10 bg-[#121d1f]/95 backdrop-blur-xl z-40 flex flex-col shadow-2xl"
  >
    <div class="p-4 border-b border-white/5 flex items-center justify-between bg-white/5">
      <div class="flex items-center gap-2">
        <Fingerprint size={16} class="text-teal-400" />
        <span class="font-bold text-slate-200 uppercase text-xs tracking-widest">Inspector</span>
      </div>
      <button onclick={onClose} class="p-1 hover:bg-white/10 rounded text-slate-400 transition-colors">
        <X size={18} />
      </button>
    </div>

    <div class="p-6 text-center border-b border-white/5 bg-gradient-to-b from-teal-500/5 to-transparent">
      <div class="w-14 h-14 bg-teal-500/10 border border-teal-500/30 rounded-full flex items-center justify-center mx-auto mb-3 shadow-[0_0_15px_rgba(20,184,166,0.1)]">
        <Database size={28} class="text-teal-400" />
      </div>
      <h3 class="text-lg font-semibold text-white truncate px-2">{nodeData.label}</h3>
      <p class="text-[10px] font-mono text-slate-500 mt-1 uppercase tracking-tighter">UID: {nodeData.id}</p>
    </div>

    <div class="grow overflow-y-auto p-4 space-y-3 custom-scrollbar">
      {#each Object.entries(nodeData.properties || {}) as [key, value]}
        <div class="group bg-black/20 border border-white/5 p-3 rounded-lg hover:border-teal-500/20 transition-all">
          <div class="flex justify-between items-center mb-1">
            <span class="text-[10px] font-bold text-teal-500/50 uppercase">{key}</span>
            <button onclick={() => copyToClipboard(String(value), key)} class="opacity-0 group-hover:opacity-100 p-1 text-slate-500 hover:text-white transition-all">
              {#if copiedKey === key}
                <Check size={12} class="text-green-400" />
              {:else}
                <Copy size={12} />
              {/if}
            </button>
          </div>
          <div class="text-sm text-slate-300 font-mono break-all leading-snug">{value}</div>
        </div>
      {/each}
    </div>

    <div class="p-4 border-t border-white/5 bg-black/20 flex flex-col gap-2">
      <button class="w-full py-2 bg-teal-500/10 hover:bg-teal-500/20 text-teal-400 text-[10px] font-bold rounded border border-teal-500/20 uppercase tracking-widest transition-all">
        Expand Links
      </button>
      <button 
        onclick={() => onDelete(nodeData)}
        class="w-full py-2 bg-red-500/10 hover:bg-red-500/20 text-red-400 text-[10px] font-bold rounded border border-red-500/20 uppercase tracking-widest flex items-center justify-center gap-2 transition-all"
      >
        <Trash2 size={12} /> Delete Node
      </button>
    </div>
  </aside>
{/if}

<style>
  .custom-scrollbar::-webkit-scrollbar { width: 3px; }
  .custom-scrollbar::-webkit-scrollbar-thumb { background: rgba(255,255,255,0.1); border-radius: 10px; }
</style>