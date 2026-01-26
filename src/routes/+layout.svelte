<script lang="ts">
  import "../app.css";
  import { onMount } from "svelte";
  import { themeProvider } from "$lib/theme.svelte";
  let { children } = $props();
  import { onOpenUrl } from '@tauri-apps/plugin-deep-link';

  onMount(async () => {
    themeProvider.apply();
    // Listen for deep link events
    await onOpenUrl((urls) => {
      const url = urls[0];
      if (url.includes('access_token')) {
        // Parse the fragment (everything after #)
        const fragment = url.split('#')[1];
        const params = new URLSearchParams(fragment);
        const token = params.get('access_token');

        if (token) {
          console.log("Token received via Deep Link:", token);
          // Now you can take this token and proceed to MFA setup or Dashboard
          localStorage.setItem("session_token", token);
          window.location.href = "/home/projects";
        }
      }
    });
  });
</script>

<main class="min-h-screen min-w-screen">
  {@render children()}
</main>
