<script lang="ts">
  import Nav from "$lib/components/Nav.svelte";
  let { children } = $props();
</script>

<main class="min-h-screen min-w-screen">
  {@render children()}
  <Nav />
</main>
