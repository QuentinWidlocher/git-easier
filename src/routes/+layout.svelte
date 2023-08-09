<script>
	import '../app.postcss';
	import { page } from '$app/stores';

	const activeClass = 'bg-sky-100 text-sky-600';
	const inactiveClass = 'bg-slate-100 text-slate-500';
</script>

<main class="h-screen flex flex-col">
	<header class="p-5 w-full mb-2">
		<nav class="grid place-content-center">
			<ul class="flex gap-5">
				<li
					class="rounded-full px-3 py-1 {$page.route.id == '/changes'
						? activeClass
						: inactiveClass}"
				>
					<a href="/changes">Your changes</a>
				</li>
				<li
					class="rounded-full px-3 py-1 {$page.route.id == '/settings'
						? activeClass
						: inactiveClass}"
				>
					<a href="/settings">Settings</a>
				</li>
			</ul>
		</nav>
	</header>
	<slot />
</main>
