<script>
    import { House, Map, Settings } from "lucide-svelte";
    import { page } from '$app/state'; // Import the page store
</script>

{#if page.url.pathname === "/home/projects" || page.url.pathname === "/home/settings" || page.url.pathname === "/home/map"}
<nav class="fixed bottom-0 left-0 right-0 flex justify-around bg-zinc-900 backdrop-blur-md px-6 py-4 rounded-t-2xl border-t border-white/10 z-50">
    
    <abbr title="Home" class="px-6 py-2 rounded-full transition-all duration-300 {page.url.pathname === "/home/projects" ? 'bg-teal-100/50 text-teal-200' : 'text-slate-400 hover:text-white'}">
        <a href="/home/projects">
            <House size="20" />
        </a>
    </abbr>

    <abbr title="Map" class="px-6 py-2 rounded-full transition-all duration-300 {page.url.pathname === "/home/map" ? 'bg-teal-100/50 text-teal-200' : 'text-slate-400 hover:text-white'}">
        <a href="/home/map">
            <Map size="20" />
        </a>
    </abbr>

           
    <abbr title="Settings" class="px-6 py-2 rounded-full transition-all duration-300 {page.url.pathname === "/home/settings" ? 'bg-teal-100/50 text-teal-200' : 'text-slate-400 hover:text-white'}">
        <a href="/home/settings">
            <Settings size="20" />
        </a>
    </abbr>
</nav>
{/if}