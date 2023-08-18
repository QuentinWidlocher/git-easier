<script>
	import '../app.postcss';
	import { page } from '$app/stores';
	import { Tabs, TabsList, TabsTrigger } from '$components/ui/tabs';

	$: currentTab = $page.route.id?.slice(1);
</script>

<main class="h-screen grid grid-cols-1 grid-rows-[5rem_1fr]">
	<header class=" z-10 h-full w-full mb-2 flex">
		<Tabs class="m-auto" value={currentTab}>
			<TabsList>
				<a href="/changes">
					<TabsTrigger value="changes">Your changes</TabsTrigger>
				</a>
				<a href="/settings">
					<TabsTrigger value="settings">Settings</TabsTrigger>
				</a>
			</TabsList>
		</Tabs>
	</header>
	<div class="h-full w-full flex flex-col overflow-y-hidden">
		<slot />
	</div>
</main>

<style>
	:root {
		--primary: 189 94% 43%;
		--muted: 210 40% 96%;
		--secondary: var(--muted);
		--accent: var(--muted);
		--radius: 1rem;
	}
</style>
