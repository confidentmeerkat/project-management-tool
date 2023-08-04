<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { open } from '@tauri-apps/api/dialog';
	import { onMount } from 'svelte';
	import ProjectItem from './project-item.svelte';
	import type { Project } from '../types';

	let projects: Array<Project> = [];

	onMount(() => {
		invoke('get_projects').then((items) => {
			projects = items as Project[];
		});
	});

	const openPicker = async () => {
		const path = await open({ directory: true, multiple: false });

		invoke('create_project', { path }).then((result) => {
			if (result === 'success') {
				invoke('get_projects').then((items) => {
					projects = items as Project[];
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

	{#each projects as project}
		<ProjectItem bind:item={project} />
	{/each}
</section>
