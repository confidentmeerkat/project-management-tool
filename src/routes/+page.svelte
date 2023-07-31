<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { open } from '@tauri-apps/api/dialog';
	import { onMount } from 'svelte';

	onMount(() => {
		invoke('get_projects').then((projects) => {
			console.log(projects);
		});
	});

	const openPicker = async () => {
		const path = await open({ directory: true, multiple: false });

		console.log(path);
	};
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
	<button on:click={openPicker}>Add Projects</button>
</section>
