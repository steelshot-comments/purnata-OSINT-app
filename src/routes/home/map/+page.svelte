<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import maplibregl from "maplibre-gl";
  import "maplibre-gl/dist/maplibre-gl.css";
  import { Plus, MapPin, Info, Layers } from "lucide-svelte";
  import { fade, slide } from "svelte/transition";

  // Svelte 5 State
  let mapContainer = $state<HTMLDivElement | null>(null);
  let map = $state<maplibregl.Map | null>(null);
  let isAddingMarker = $state(false);
  let zoomLvl = $state(2);
  let isMenuOpen = $state(false);
  let showModal = $state(false);

  onMount(() => {
    if (!mapContainer) return;

    map = new maplibregl.Map({
      container: mapContainer,
      style: "https://demotiles.maplibre.org/style.json", // Replace with your Mapbox URL if needed
      center: [-98.0, 39.5],
      zoom: zoomLvl,
    });

    map.on("zoom", () => {
      zoomLvl = map!.getZoom();
    });

    map.on("click", (e) => {
      if (isAddingMarker) {
        addMarker(e.lngLat);
      }
    });
  });

  function addMarker(lngLat: maplibregl.LngLat) {
    if (!map) return;

    // Create a custom element for the marker (mimics your .webp asset)
    const el = document.createElement("div");
    el.className = "custom-marker";
    el.innerHTML = '<span style="font-size: 24px;">📍</span>';

    new maplibregl.Marker({ element: el }).setLngLat(lngLat).addTo(map);

    // Optional: Turn off adding mode after placement like some mobile apps
    // isAddingMarker = false;
  }

  onDestroy(() => {
    map?.remove();
  });
</script>

<div class="relative w-full h-screen bg-slate-900 overflow-hidden">
  <div bind:this={mapContainer} class="w-full h-full"></div>

  <div
    class="absolute top-4 left-4 bg-black/50 backdrop-blur-md px-3 py-1 rounded-full text-white text-xs z-10"
  >
    Zoom: {zoomLvl.toFixed(1)}
  </div>

  <div class="floating flex flex-col-reverse items-end gap-4 z-20">
    <button
      onclick={() => (isMenuOpen = !isMenuOpen)}
      class="floating-button transition-transform active:scale-95 {isMenuOpen
        ? 'rotate-45'
        : ''}"
    >
      <Plus size={24} />
    </button>

    {#if isMenuOpen}
      <div
        class="flex flex-col-reverse items-center gap-3"
        transition:fade={{ duration: 150 }}
      >
        <button
          onclick={() => {
            isAddingMarker = !isAddingMarker;
            isMenuOpen = false;
          }}
          class="flex items-center gap-3 group"
        >
          <span
            class="bg-slate-800 text-white px-3 py-1 rounded-lg text-sm opacity-0 group-hover:opacity-100 transition-opacity shadow-lg"
          >
            {isAddingMarker ? "Cancel Adding" : "Add Marker"}
          </span>
          <div
            class="w-12 h-12 rounded-full {isAddingMarker
              ? 'bg-red-500'
              : 'bg-white text-slate-900'} shadow-lg flex items-center justify-center"
          >
            <MapPin size={20} />
          </div>
        </button>

        <button
          onclick={() => {
            showModal = true;
            isMenuOpen = false;
          }}
          class="flex items-center gap-3 group"
        >
          <span
            class="bg-slate-800 text-white px-3 py-1 rounded-lg text-sm opacity-0 group-hover:opacity-100 transition-opacity shadow-lg"
          >
            Open Modal
          </span>
          <div
            class="w-12 h-12 rounded-full bg-white text-slate-900 shadow-lg flex items-center justify-center"
          >
            <Info size={20} />
          </div>
        </button>
      </div>
    {/if}
  </div>

  {#if showModal}
    <button
      aria-label="close"
      onclick={() => (showModal = false)}
      class="absolute inset-0 bg-black/40 z-30"
      transition:fade
    ></button>

    <div
      class="absolute bottom-0 left-0 right-0 bg-slate-900 rounded-t-3xl p-8 z-40 border-t border-white/10 shadow-2xl"
      transition:slide={{ axis: "y" }}
    >
      <div class="w-12 h-1.5 bg-white/20 rounded-full mx-auto mb-6"></div>
      <div class="text-center space-y-4">
        <h2 class="text-xl font-bold text-white">Modal BottomSheet</h2>
        <p class="text-slate-400">
          This is the Svelte equivalent of your Flutter BottomSheet.
        </p>
        <button
          onclick={() => (showModal = false)}
          class="w-full py-3 bg-teal-500 text-white rounded-xl font-bold"
        >
          Close BottomSheet
        </button>
      </div>
    </div>
  {/if}

  {#if isAddingMarker}
    <div
      class="absolute top-4 left-1/2 -translate-x-1/2 bg-teal-500 text-white px-6 py-2 rounded-full shadow-2xl z-10 animate-bounce"
    >
      Tap anywhere to place a marker
    </div>
  {/if}
</div>

<style>
  /* Ensure the map container fills the space */
  :global(.maplibregl-canvas) {
    outline: none;
  }
</style>
