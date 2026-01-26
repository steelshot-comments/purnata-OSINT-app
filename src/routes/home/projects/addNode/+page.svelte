<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { ArrowLeft, X, Trash2, Plus } from "lucide-svelte";
  import { graphState } from '$lib/graph.svelte';

  // Fixed dummy values for labels
  const availableLabels = ["Email", "Person", "Project", "Server"];

  // Svelte 5 State
  let selectedLabel = $state("Email");
  let properties = $state([
    { key: "address", value: "yeshayaav@gmail.com" },
    { key: "provider", value: "google" },
  ]);

  function addProperty() {
    properties.push({ key: "", value: "" });
  }

  function removeProperty(index: number) {
    properties = properties.filter((_, i) => i !== index);
  }

  async function handleSubmit() {
    try {
      // Convert array to a standard object for Rust
      const nodeData = properties.reduce(
        (acc, prop) => {
          if (prop.key) acc[prop.key] = prop.value;
          return acc;
        },
        {} as Record<string, string>,
      );

      console.log("Submitting Node:", {
        label: selectedLabel,
        properties: nodeData,
      });

      // Invoke the Rust function
      const response = await invoke("add_node_to_graph", {
        label: selectedLabel,
        properties: nodeData,
      });

      console.log("Rust Response:", response);
      graphState.needsRefresh = true;
      history.back();
    } catch (error) {
      console.error("Failed to add node:", error);
      alert("Error adding node: " + error);
    }
  }
</script>

<div class="min-h-screen bg-[#12181b] text-slate-200 flex flex-col font-sans">
  <header
    class="h-16 flex items-center px-4 border-b border-white/5 bg-[#12181b] sticky top-0 z-10"
  >
    <button
      onclick={() => history.back()}
      class="p-2 hover:bg-white/5 rounded-full transition-colors"
    >
      <ArrowLeft size={24} />
    </button>
    <h1 class="ml-4 text-xl font-medium">Add Nodes</h1>
  </header>

  <main class="grow p-6 max-w-2xl w-full mx-auto space-y-8">
    <div class="space-y-6">
      <div class="flex items-center justify-between">
        <h2 class="text-slate-400 text-sm font-medium uppercase tracking-wider">
          New node
        </h2>
        <button class="text-teal-500/80 hover:text-teal-400 transition-colors">
          <X size={24} />
        </button>
      </div>

      <div class="relative group">
        <label
          for="label-select"
          class="absolute -top-2.5 left-3 px-1 bg-[#12181b] text-xs text-slate-500 z-10"
        >
          Label
        </label>
        <select
          id="label-select"
          bind:value={selectedLabel}
          class="w-full bg-[#2a2e33]/50 border border-white/10 rounded-lg p-4 outline-none focus:border-teal-500/50 appearance-none text-lg"
        >
          {#each availableLabels as label}
            <option value={label}>{label}</option>
          {/each}
        </select>
        <div
          class="absolute right-4 top-1/2 -translate-y-1/2 pointer-events-none text-slate-500"
        >
          ▼
        </div>
      </div>

      <div class="space-y-6">
        {#each properties as prop, i}
          <div class="flex items-start gap-3 group">
            <div class="grow relative">
              <p
                class="absolute -top-2.5 left-3 px-1 bg-[#12181b] text-xs text-teal-500 z-10"
              >
                {prop.key || "property"}
              </p>
              <input
                type="text"
                bind:value={prop.value}
                placeholder="Value"
                class="w-full bg-transparent border-2 border-teal-500/30 rounded-lg p-4 outline-none focus:border-teal-500 transition-colors"
              />
            </div>
            <button
              onclick={() => removeProperty(i)}
              class="mt-4 p-2 text-slate-500 hover:text-red-400 transition-colors"
            >
              <Trash2 size={20} />
            </button>
          </div>
        {/each}
      </div>
    </div>
  </main>

  <footer class="p-6 bg-[#12181b] border-t border-white/5 flex gap-4">
    <button
      onclick={addProperty}
      class="flex-1 py-3 bg-teal-500/10 text-teal-400 font-semibold rounded-xl hover:bg-teal-500/20 transition-all active:scale-[0.98]"
    >
      Add another property
    </button>
    <button
      onclick={handleSubmit}
      class="flex-1 py-3 bg-teal-500/20 text-teal-400 font-semibold rounded-xl border border-teal-500/30 hover:bg-teal-500/30 transition-all active:scale-[0.98]"
    >
      Submit
    </button>
  </footer>
</div>

<style>
  /* Fix for select arrow on different browsers */
  select {
    background-image: none;
  }
</style>
