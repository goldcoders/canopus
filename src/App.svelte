<script lang="ts">
	export let name: string;
</script>

<main class="flex mx-auto max-w-5xl items-center justify-center min-h-screen">
	<h1 class="text-red-400 font-bold text-5xl">{name}</h1>
</main>

<style global>
  @tailwind base;
  @tailwind components;
  @tailwind utilities;
</style>
