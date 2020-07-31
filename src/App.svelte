<script lang="ts">
	export let name: string;
	import Main from "./partials/main.svelte";
	import { promisified } from 'tauri/api/tauri';
	import { onMount } from 'svelte';
	import AlertError from "./partials/alert-error.svelte";
	import AlertSuccess from "./partials/alert-success.svelte";
	let errorMessage: string;
	let message: string;

    onMount( () => {
        promisified({
        cmd: 'doSomething',
        count: 6,
        payload: {
            state: 'some string data',
            data: 17
        }
        }).then(response => {
        // do something with the Ok() response
        message = response.message

        }).catch(error => {
        // do something with the Err() response string
        errorMessage = error
        console.log(error)
        })
	});


</script>
{#if errorMessage}
<AlertError {errorMessage}/>
{/if}

{#if message}
<AlertSuccess {message}/>
{/if}

<Main {name}/>

<style global>
  @tailwind base;
  @tailwind components;
  @tailwind utilities;
</style>

<svelte:head>
<link rel="stylesheet" href="/fonts/inter.css" />
</svelte:head>
