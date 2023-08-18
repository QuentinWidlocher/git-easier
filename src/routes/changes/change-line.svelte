<script lang="ts">
	import type { GitFile } from '$lib/types/gitfile';
	import { moveFile } from '$lib/services/back';
	import { invalidate } from '$app/navigation';
	import { Button } from '$components/ui/button';
	import { ChevronLeft, ChevronRight } from 'lucide-svelte';

	export let file: GitFile;
	export let arrowPosition: 'right' | 'left';
	export let disabled: boolean = false;

	$: path = file.path.split('/').slice(0, -1).join('/');
	$: filename = file.path.split('/').pop();

	async function onMoveClick() {
		if (disabled) {
			return;
		}

		await moveFile({ path: file.path, action: arrowPosition === 'left' ? 'unstage' : 'stage' });
		await invalidate('changes:list');
	}
</script>

{#if arrowPosition === 'left'}
	<Button
		variant="ghost"
		class="h-auto rounded-r-none rounded-l-xl hover:bg-primary hover:text-primary-foreground"
		on:click={onMoveClick}
		{disabled}
	>
		<ChevronLeft />
	</Button>
{/if}

<span class="font-mono p-2 flex flex-col overflow-x-hidden">
	<span class="opacity-60 text-sm overflow-x-hidden overflow-ellipsis">{path}/</span>
	<span>{filename}</span>
</span>

{#if arrowPosition === 'right'}
	<Button
		variant="ghost"
		class="h-auto rounded-l-none rounded-r-xl focus:bg-primary focus:text-primary-foreground hover:bg-primary hover:text-primary-foreground"
		on:click={onMoveClick}
		{disabled}
	>
		<ChevronRight />
	</Button>
{:else}
	<div class="flex-1" />
{/if}
