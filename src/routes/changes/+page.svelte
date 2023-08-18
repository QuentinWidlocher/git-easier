<script lang="ts">
	import ChangeColumn from './change-column.svelte';
	import { publish, syncWithSource } from '$lib/services/back';
	import type { PageData } from './$types';
	import { invalidate } from '$app/navigation';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$components/ui/input';
	import { ArrowUpFromLine, RefreshCw } from 'lucide-svelte';

	export let data: PageData;

	let message = '';
	let publishing = false;
	let syncing = false;

	$: locked = publishing || syncing;

	async function handlePublish() {
		if (data.staged.length === 0) return;

		publishing = true;

		queueMicrotask(async () => {
			await publish({ message: message || undefined });
			await invalidate('changes:list');

			publishing = false;
			message = '';
		});
	}

	async function handleSync() {
		syncing = true;
		queueMicrotask(async () => {
			await syncWithSource();
			await invalidate('changes:list');
			syncing = false;
		});
	}
</script>

<Button variant="secondary" class="mx-auto" on:click={handleSync} disabled={locked}>
	<RefreshCw class="mr-2 h-4 w-4 {syncing ? 'animate-spin' : 'no'}" />
	{syncing ? 'Syncing...' : 'Sync with source'}
</Button>
<div class="p-5 grid grid-rows-[1fr_auto] grid-cols-2 gap-5 flex-1 overflow-y-hidden">
	<ChangeColumn
		title="Local changes ({data.unstaged.length})"
		files={data.unstaged}
		status="local"
		disabled={locked}
	/>
	<ChangeColumn
		title="Changes ready to publish ({data.staged.length})"
		files={data.staged}
		disabled={locked}
		status="staged"
	/>

	<form on:submit={handlePublish} class="col-span-2 flex">
		<Input
			class="flex-1 rounded-r-none border-r-none"
			placeholder="Add a small message to your changes"
			name="message"
			type="text"
			bind:value={message}
		/>
		<Button class="rounded-l-none" disabled={locked || data.staged.length === 0}>
			{#if publishing}
				<RefreshCw class="mr-2 h-4 w-4 animate-spin" />
			{:else}
				<ArrowUpFromLine class="mr-2 h-4 w-4" />
			{/if}
			{publishing ? 'Publishing...' : 'Publish these changes'}
		</Button>
	</form>
</div>
