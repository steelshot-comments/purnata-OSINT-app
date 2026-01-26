<script lang="ts">
	import { User, ChevronDown, LogOut, FileText, Bell, Shield, Settings } from 'lucide-svelte';
	import { goto } from '$app/navigation';
    import { themeProvider } from '$lib/theme.svelte';
	import {invoke} from '@tauri-apps/api/core';

	// Svelte 5 Runes for state
	let enableNotifications = $state(true);
	let notificationPosition = $state('Top-Right');
	let incognitoMode = $state(false);
	let autoLogout = $state(false);

	const themeOptions = ['system', 'light', 'dark'];
	const positionOptions = ['Top-Right', 'Bottom-Left', 'Center'];

	async function handleLogout() {
		// Replace with your actual logout logic/Tauri invoke
		console.log('Logging out...');
		await invoke('logout');
		goto('/');
	}
</script>

<div class="min-h-screen bg-[#1a2332] text-white flex flex-col font-sans">
	<header class="h-16 flex items-center justify-between px-6 border-b border-white/10 shrink-0 bg-[#1a2332]">
		<h1 class="text-xl font-semibold">Welcome Yeshaya!</h1>
		<button 
			onclick={() => goto('/account')}
			class="p-2 hover:bg-white/10 rounded-full transition-colors"
		>
			<User size={24} />
		</button>
	</header>

	<main class="grow overflow-y-auto p-4 space-y-6 max-w-2xl mx-auto w-full">
		
		<section>
			<h2 class="px-2 mb-2 text-lg font-bold flex items-center gap-2">
				<Settings size={18} class="text-teal-400" /> General
			</h2>
			<div class="bg-white/5 rounded-2xl overflow-hidden divide-y divide-white/5">
				<div class="flex items-center justify-between p-4">
					<span>Theme Mode</span>
					<select 
						bind:value={themeProvider.mode}
                        onchange={(e)=>{themeProvider.setTheme(e.currentTarget.value as any)}}
						class="rounded-lg px-3 py-1 text-sm outline-none focus:ring-2 focus:ring-teal-500"
					>
						{#each themeOptions as option}
							<option value={option}>{option}</option>
						{/each}
					</select>
				</div>
			</div>
		</section>

		<section>
			<h2 class="px-2 mb-2 text-lg font-bold flex items-center gap-2">
				<Bell size={18} class="text-amber-400" /> Notifications & Alerts
			</h2>
			<div class="bg-white/5 rounded-2xl overflow-hidden divide-y divide-white/5">
				<label class="flex items-center justify-between p-4 cursor-pointer hover:bg-white/5 transition-colors">
					<span>Enable Notifications</span>
					<input type="checkbox" bind:checked={enableNotifications} class="sr-only peer" />
					<div class="w-11 h-6 bg-slate-700 rounded-full peer peer-checked:after:translate-x-full after:content-[''] after:absolute after:top-0.5 after:left-0.5 after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-teal-500 relative"></div>
				</label>

				<div class="flex items-center justify-between p-4">
					<span>Notification Position</span>
					<select 
						bind:value={notificationPosition}
						class="bg-slate-800 border border-white/10 rounded-lg px-3 py-1 text-sm outline-none"
					>
						{#each positionOptions as option}
							<option value={option}>{option}</option>
						{/each}
					</select>
				</div>
			</div>
		</section>

		<section>
			<h2 class="px-2 mb-2 text-lg font-bold flex items-center gap-2">
				<Shield size={18} class="text-red-400" /> Security & Privacy
			</h2>
			<div class="bg-white/5 rounded-2xl overflow-hidden divide-y divide-white/5">
				<label class="flex items-center justify-between p-4 cursor-pointer hover:bg-white/5">
					<span>Incognito Mode</span>
					<input type="checkbox" bind:checked={incognitoMode} class="sr-only peer" />
					<div class="w-11 h-6 bg-slate-700 rounded-full peer peer-checked:after:translate-x-full after:content-[''] after:absolute after:top-0.5 after:left-0.5 after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-teal-500 relative"></div>
				</label>

				<label class="flex items-center justify-between p-4 cursor-pointer hover:bg-white/5">
					<span>Auto Logout</span>
					<input type="checkbox" bind:checked={autoLogout} class="sr-only peer" />
					<div class="w-11 h-6 bg-slate-700 rounded-full peer peer-checked:after:translate-x-full after:content-[''] after:absolute after:top-0.5 after:left-0.5 after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-teal-500 relative"></div>
				</label>
			</div>
		</section>

		<div class="flex gap-4 pt-4">
			<button 
				onclick={() => goto('/home/settings/policies')}
				class="flex-1 flex items-center justify-center gap-2 py-4 bg-white/10 hover:bg-white/20 rounded-3xl font-semibold transition-all active:scale-95"
			>
				<FileText size={18} /> Policies
			</button>
			<button 
				onclick={handleLogout}
				class="flex-1 flex items-center justify-center gap-2 py-4 bg-red-500/20 hover:bg-red-500/30 text-red-400 rounded-3xl font-semibold transition-all active:scale-95 border border-red-500/20"
			>
				<LogOut size={18} /> Log out
			</button>
		</div>
	</main>
</div>

<style>
	/* Custom checkbox / Switch styling is handled by Tailwind classes above */
</style>