<script lang="ts">
	import { invalidate } from '$app/navigation';
	import { moveAllFile } from '$lib/services/back';
	import type { GitFile } from '$lib/types/gitfile';
	import ChangeLine from './change-line.svelte';
	import {
		Card,
		CardContent,
		CardDescription,
		CardFooter,
		CardHeader,
		CardTitle
	} from '$components/ui/card';
	import { Button } from '$components/ui/button';
	import { ChevronsLeft, ChevronsRight } from 'lucide-svelte';

	export let title: string;
	export let files: Array<GitFile>;
	export let status: 'local' | 'staged';
	export let disabled: boolean = false;

	async function onMoveAllClick() {
		if (disabled) {
			return;
		}

		await moveAllFile({ action: status == 'local' ? 'stage' : 'unstage' });
		await invalidate('changes:list');
	}
</script>

<Card class="overflow-y-auto flex flex-col">
	<CardHeader>
		<CardTitle
			class="flex gap-5 justify-between items-center {status == 'staged' ? 'flex-row-reverse' : ''}"
		>
			<span>{title}</span>
			<Button variant="ghost" {disabled} on:click={onMoveAllClick}>
				{#if status == 'local'}
					Ready all files <ChevronsRight class="ml-2 h-4 w-4" />
				{:else}
					<ChevronsLeft class="mr-2 h-4 w-4" /> Unready all files
				{/if}
			</Button>
		</CardTitle>
	</CardHeader>
	<CardContent>
		<ul class="flex-1 flex flex-col gap-2">
			{#each files as file}
				<li class="flex justify-between hover:bg-muted rounded-xl overflow-x-hidden">
					<ChangeLine {disabled} {file} arrowPosition={status == 'local' ? 'right' : 'left'} />
				</li>
			{/each}
		</ul>
	</CardContent>
	<CardFooter />
</Card>
