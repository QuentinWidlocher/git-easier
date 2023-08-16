<script lang="ts">
	import ChangeColumn from './change-column.svelte';
	import { publish, syncWithSource } from '$lib/services/back';
	import type { PageData } from './$types';
	import { invalidate } from '$app/navigation';
	import { listen } from '@tauri-apps/api/event';

	export let data: PageData;

	let message = '';
	let publishing = false;
	let syncing = false;

	async function handlePublish() {
		if (data.staged.length === 0) return;

		publishing = true;

		await publish({ message: message || undefined });
		await invalidate('changes:list');

		publishing = false;
		message = '';
	}

	async function handleSync() {
		syncing = true;
		await syncWithSource();
		await invalidate('changes:list');
		syncing = false;
	}
</script>

<button
	class="mx-auto px-3 py-2 rounded-full bg-sky-50 text-sky-500 border transform active:translate-y-px border-sky-500/20 hover:bg-sky-200 active:bg-sky-300 active:text-sky-600 hover:border-sky-400"
	class:animate-pulse={syncing}
	on:click={handleSync}
	disabled={publishing || syncing}
>
	{syncing ? '🔄 Syncing...' : '🔄 Sync with source'}
</button>
<div class="p-5 grid grid-rows-[1fr_auto] grid-cols-2 gap-5 flex-1 overflow-y-hidden">
	<ChangeColumn
		title="Local changes ({data.unstaged.length})"
		files={data.unstaged}
		status="local"
	/>
	<ChangeColumn
		title="Changes ready to publish ({data.staged.length})"
		files={data.staged}
		status="staged"
	/>

	<form on:submit={handlePublish} class="col-span-2 flex gap-5">
		<input
			class="flex-1 rounded-full border border-slate-400 pl-5 pr-3 py-2"
			placeholder="Add a small message to your changes"
			name="message"
			type="text"
			bind:value={message}
		/>
		<button
			class:animate-pulse={publishing}
			class=" px-5 py-2 rounded-full bg-sky-500 text-white border transform active:translate-y-px border-sky-400 hover:bg-sky-600 active:bg-sky-700 active:text-sky-100 hover:border-sky-500"
			disabled={publishing || syncing || data.staged.length === 0}
		>
			{publishing ? 'Publishing...' : 'Publish these changes'}
		</button>
	</form>
</div>
