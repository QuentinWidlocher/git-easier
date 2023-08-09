<script lang="ts">
	import ChangeColumn from './change-column.svelte';
	import { publish } from '$lib/services/back';
	import type { PageData } from './$types';
	import { invalidate } from '$app/navigation';
	import { listen } from '@tauri-apps/api/event';

	export let data: PageData;

	let message = '';

	async function handlePublish() {
		if (data.staged.length === 0) return;

		await publish({ message: message || undefined });
		await invalidate('changes:list');
		message = '';
	}

	listen('tauri://focus', () => {
		invalidate('changes:list');
	});
</script>

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
			class="px-5 py-2 rounded-full bg-sky-500 text-white border transform active:translate-y-px border-sky-400 hover:bg-sky-600 active:bg-sky-700 active:text-sky-100 hover:border-sky-500"
			disabled={data.staged.length === 0}
		>
			Publish these changes
		</button>
	</form>
</div>
