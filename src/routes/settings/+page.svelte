<script lang="ts">
	import { Button } from '$components/ui/button';
	import { Input } from '$components/ui/input';
	import { Label } from '$components/ui/label';
	import { Save } from 'lucide-svelte';
	import type { PageData } from './$types';
	export let data: PageData;

	let saved = false;

	async function handleSubmit(
		event: Event & { readonly submitter: HTMLElement | null } & {
			currentTarget: EventTarget & HTMLFormElement;
		}
	) {
		const form = new FormData(event.currentTarget);

		await Promise.all([
			data.store.set('repo-path', form.get('repoPath')),
			data.store.set('working-branch', form.get('working-branch')),
			data.store.set('source-branch', form.get('source-branch'))
		]);

		saved = true;
		setTimeout(() => (saved = false), 2000);
	}
</script>

<section class="grid place-content-center h-full -mt-[5rem]">
	<form on:submit={handleSubmit} class="w-96 flex flex-col gap-5">
		<div class="flex flex-col gap-2">
			<Label for="repoPath">Path to the git repository</Label>
			<Input type="text" placeholder="/path/to/repo" name="repoPath" bind:value={data.repoPath} />
		</div>
		<div class="flex flex-col gap-2">
			<Label for="working-branch">Branch to work on</Label>
			<Input
				type="text"
				placeholder="develop"
				name="working-branch"
				bind:value={data.workingBranch}
			/>
		</div>
		<div class="flex flex-col gap-2">
			<Label for="source-branch">Branch to sync with</Label>
			<Input type="text" placeholder="main" name="source-branch" bind:value={data.sourceBranch} />
		</div>
		<Button>
			<Save class="mr-2 h-4 w-4" />
			{saved ? 'Saved!' : 'Save settings'}
		</Button>
	</form>
</section>
