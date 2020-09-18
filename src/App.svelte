<style global>
@tailwind base;
@tailwind components;
@tailwind utilities;
</style>

<script lang="ts">
export let name: string;
import Main from "./partials/main.svelte";
</script>

<svelte:head>
  <link rel="stylesheet" href="/fonts/inter.css" />
</svelte:head>

<Main name="{name}" />
