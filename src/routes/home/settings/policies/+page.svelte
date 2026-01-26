<script lang="ts">
  import { onMount } from 'svelte';
  import { marked } from 'marked';
  import { ChevronLeft, BookOpen } from 'lucide-svelte';

  let mdContent = $state('');
  let isLoading = $state(true);

  onMount(async () => {
    try {
      // If your file is at static/docs/privacy.md, the fetch path is '/docs/privacy.md'
      const response = await fetch('/policies.md'); 
      if (!response.ok) throw new Error('File not found');
      
      const text = await response.text();
      mdContent = await marked.parse(text);
    } catch (err) {
      mdContent = `<p class="text-red-400">Error loading document: ${err.message}</p>`;
    } finally {
      isLoading = false;
    }
  });
</script>

<div class="min-h-screen bg-[#1a2332] text-slate-200">
  <header class="h-16 flex items-center px-6 border-b border-white/10 bg-[#1a2332]/80 backdrop-blur-md sticky top-0 z-20">
    <button 
      onclick={() => history.back()} 
      class="p-2 hover:bg-white/10 rounded-full mr-4 transition-colors"
    >
      <ChevronLeft size={24} />
    </button>
    <div class="flex items-center gap-2">
      <BookOpen size={20} class="text-teal-400" />
      <h1 class="font-semibold text-lg">Documentation</h1>
    </div>
  </header>

  <main class="max-w-4xl mx-auto p-6 md:p-12">
    {#if isLoading}
      <div class="flex flex-col items-center justify-center py-20 animate-pulse">
        <div class="w-12 h-12 border-4 border-teal-500/20 border-t-teal-500 rounded-full animate-spin"></div>
        <p class="mt-4 text-slate-400">Reading document...</p>
      </div>
    {:else}
      <div class="bg-white/5 border border-white/10 rounded-3xl p-8 md:p-12 shadow-2xl">
        <article class="prose prose-invert prose-teal max-w-none">
          {@html mdContent}
        </article>
      </div>
    {/if}
  </main>
</div>

<!-- <style>
  /* Custom prose overrides to match your dark theme perfectly */
  :global(.prose h1) { @apply text-4xl font-extrabold mb-8 text-white border-b border-white/10 pb-4; }
  :global(.prose h2) { @apply text-2xl font-bold mt-10 mb-4 text-teal-400; }
  :global(.prose p) { @apply text-slate-300 leading-relaxed mb-6 text-lg; }
  :global(.prose code) { @apply bg-slate-800 text-teal-300 px-1.5 py-0.5 rounded font-mono text-sm; }
  :global(.prose pre) { @apply bg-slate-900 border border-white/5 rounded-xl p-4 my-6; }
  :global(.prose ul) { @apply list-disc list-inside space-y-2 mb-6 text-slate-300; }
  :global(.prose strong) { @apply text-white font-semibold; }
</style> -->