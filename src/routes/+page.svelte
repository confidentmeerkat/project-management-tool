<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { open } from '@tauri-apps/api/dialog';
	import { onMount } from 'svelte';

	let projects = [];

	onMount(() => {
		invoke('get_projects').then((items) => {
			projects = items;
		});
	});

	const openPicker = async () => {
		const path = await open({ directory: true, multiple: false });

		invoke('create_project', { path }).then((result) => {
			if (result === 'success') {
				invoke('get_projects').then((items) => {
					console.log(items);
					projects = items;
				});
			}
		});
	};
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
	<button on:click={openPicker}>Add Projects</button>
</section>
