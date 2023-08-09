<script lang="ts">
	import type { PageData } from './$types';
	export let data: PageData;

	let saved = false;

	async function handleSubmit(
		event: Event & { readonly submitter: HTMLElement | null } & {
			currentTarget: EventTarget & HTMLFormElement;
		}
	) {
		const form = new FormData(event.currentTarget);

		console.log(...form.values());

		await Promise.all([
			data.store.set('repo-path', form.get('repoPath')),
			data.store.set('branch', form.get('branch'))
		]);

		saved = true;
		setTimeout(() => (saved = false), 2000);
	}
</script>

<form on:submit={handleSubmit} class="mt-5 mx-auto w-96 flex flex-col gap-5">
	<div class="flex flex-col">
		<label class="text-slate-500 ml-6" for="repoPath">Path to the git repository</label>
		<input
			class="flex-1 rounded-full border border-slate-400 pl-5 pr-3 py-2"
			type="text"
			placeholder="/path/to/repo"
			name="repoPath"
			bind:value={data.repoPath}
		/>
	</div>
	<div class="flex flex-col">
		<label class="text-slate-500 ml-6" for="branch">Branch to work on</label>
		<input
			class="flex-1 rounded-full border border-slate-400 pl-5 pr-3 py-2"
			type="text"
			placeholder="main"
			name="branch"
			bind:value={data.branch}
		/>
	</div>
	<button
		class="px-5 py-2 rounded-full bg-sky-500 text-white border transform active:translate-y-px border-sky-400 hover:bg-sky-600 active:bg-sky-700 active:text-sky-100 hover:border-sky-500"
	>
		{saved ? 'Saved!' : 'Save settings'}
	</button>
</form>
