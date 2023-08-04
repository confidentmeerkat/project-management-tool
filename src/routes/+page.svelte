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

<section class="p-2">
	<div class="flex flex-row justify-between my-2">
		<h1>Projects</h1>
		<button on:click={openPicker} class="rounded-sm bg-blue-600 text-white py-1.5 px-3"
			>Add Projects</button
		>
	</div>

	<ul class="space-y-2">
		{#each projects as project}
			<ProjectItem bind:item={project} />
		{/each}
	</ul>
</section>
