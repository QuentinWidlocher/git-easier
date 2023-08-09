<script lang="ts">
	import { invalidate } from '$app/navigation';
	import { moveAllFile } from '$lib/services/back';
	import type { GitFile } from '$lib/types/gitfile';
	import ChangeLine from './change-line.svelte';

	export let title: string;
	export let files: Array<GitFile>;
	export let status: 'local' | 'staged';

	const allClasses = {
		local: {
			cardTitle: 'text-slate-500',
			cardBg: 'bg-slate-50/50',
			lineHover: 'hover:bg-slate-400/30'
		},
		staged: {
			cardTitle: 'text-sky-500',
			cardBg: 'bg-sky-50/50',
			lineHover: 'hover:bg-sky-400/30'
		}
	};

	const classes = allClasses[status];

	async function onMoveAllClick() {
		await moveAllFile({ action: status == 'local' ? 'stage' : 'unstage' });
		await invalidate('changes:list');
	}
</script>

<div class="overflow-y-hidden flex flex-col">
	<div
		class:flex-row-reverse={status == 'staged'}
		class="mx-10 mb-5 flex items-center justify-between"
	>
		<h2 class="text-lg {classes.cardTitle} ">{title}</h2>

		<button
			class="px-3 py-2 rounded-full bg-sky-50 text-sky-500 border transform active:translate-y-px border-sky-500/20 hover:bg-sky-200 active:bg-sky-300 active:text-sky-600 hover:border-sky-400"
			on:click={onMoveAllClick}
		>
			{#if status == 'local'}
				Ready all files ▶️
			{:else}
				◀️ Unready all files
			{/if}
		</button>
	</div>
	<div class="flex-1 overflow-y-auto rounded-3xl {classes.cardBg}  p-5">
		<ul>
			{#each files as file}
				<li class="flex justify-between {classes.lineHover} rounded-xl overflow-x-hidden">
					<ChangeLine {file} arrowPosition={status == 'local' ? 'right' : 'left'} />
				</li>
			{/each}
		</ul>
	</div>
</div>
