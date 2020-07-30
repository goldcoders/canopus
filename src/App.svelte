<script lang="ts">
	export let name: string;
	import Main from "./partials/main.svelte";
</script>

<Main {name}/>

<style global>
  @tailwind base;
  @tailwind components;
  @tailwind utilities;
</style>

<svelte:head>
<link rel="stylesheet" href="/fonts/inter.css" />
</svelte:head>
