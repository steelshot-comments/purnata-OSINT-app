<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Loader2, Play } from "lucide-svelte";

  let { text, source, query, nodeID } = $props<{
    text: string;
    source: string;
    query: string;
    nodeID: string;
  }>();

  let isLoading = $state(false);
  let wsMessage = $state('');
  let socket: WebSocket | null = null;

  async function startTransform() {
    isLoading = true;
    wsMessage = '';

    try {
      // 1. Trigger the Rust command that hits your POST endpoint
      // Assuming you added 'run_transform' to your Rust backend
      await invoke("run_transform", { 
        source: source, 
        nodeId: nodeID, 
        query: query 
      });

      // 2. Connect to WebSocket
      // Note: Use your environment variable for the URL
      const wsUrl = `ws://your-production-url/ws/transforms/${nodeID}`;
      socket = new WebSocket(wsUrl);

      socket.onmessage = (event) => {
        wsMessage = event.data;
        isLoading = false;
        socket?.close();
      };

      socket.onerror = (error) => {
        console.error("WS Error:", error);
        isLoading = false;
      };

    } catch (e) {
      console.error("Transform Error:", e);
      isLoading = false;
    }
  }
</script>

<div class="flex items-center gap-3 bg-white/5 p-2 rounded-lg border border-white/10 group">
  <button 
    onclick={startTransform}
    disabled={isLoading}
    class="flex items-center gap-2 px-3 py-1.5 bg-teal-500/20 hover:bg-teal-500/40 text-teal-400 text-xs font-bold rounded transition-all disabled:opacity-50"
  >
    {#if isLoading}
      <Loader2 size={14} class="animate-spin" />
    {:else}
      <Play size={12} fill="currentColor" />
    {/if}
    {text}
  </button>
  
  <span class="text-xs text-slate-400 italic truncate grow">
    {wsMessage || (isLoading ? 'Processing...' : source)}
  </span>
</div>