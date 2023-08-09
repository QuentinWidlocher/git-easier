<script lang="ts">
	import type { GitFile } from '$lib/types/gitfile';
	import { moveFile } from '$lib/services/back';
	import { invalidate } from '$app/navigation';

	export let file: GitFile;
	export let arrowPosition: 'right' | 'left';

	$: path = file.path.split('/').slice(0, -1).join('/');
	$: filename = file.path.split('/').pop();

	async function onMoveClick() {
		await moveFile({ path: file.path, action: arrowPosition === 'left' ? 'unstage' : 'stage' });
		await invalidate('changes:list');
	}
</script>

{#if arrowPosition === 'left'}
	<button class="p-4 hover:bg-white/30 rounded-l-xl" on:click={onMoveClick}>◀️</button>
{/if}

<span class="font-mono p-2 flex flex-col overflow-x-hidden">
	<span class="opacity-60 text-sm overflow-x-hidden overflow-ellipsis">{path}/</span>
	<span>{filename}</span>
</span>

{#if arrowPosition === 'right'}
	<button class="p-4 hover:bg-white/30 rounded-r-xl" on:click={onMoveClick}>▶️</button>
{:else}
	<div class="flex-1" />
{/if}
