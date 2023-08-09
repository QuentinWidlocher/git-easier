<script lang="ts">
	import { page } from '$app/stores';
</script>

<div class="grid place-content-center">
	<h1 class="text-red-800 text-2xl">Error</h1>
	<p class="my-2 text-red-500">{$page.error}</p>

	<details>
		<summary>See more</summary>

		<pre>{JSON.stringify($page.error, null, 2)}</pre>
	</details>
</div>
