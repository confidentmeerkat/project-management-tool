<script lang="ts">
	import { onMount } from 'svelte';
	import ProjectItem from './project-item.svelte';
	import { Button, Heading, Listgroup } from 'flowbite-svelte';
	import NewModal from '$lib/components/projects/new-modal.svelte';
	import { getProjects } from '$lib/api/tauri/projects';
	import projects from '$lib/store/projects';

	export let data;

	onMount(() => {
		getProjects().then((items) => {
			projects.set(items);
		});
	});

	const handleOpenModal = () => {
		openNewModal = true;
	};
	const handleCloseModal = () => openNewModal = false;

	let openNewModal = false;
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section class="p-2">
	<div class="flex flex-row justify-between my-2">
		<Heading tag="h4">Projects</Heading>
		<Button class="whitespace-nowrap" on:click={handleOpenModal}>Add Projects</Button>
	</div>

	<Listgroup class="rounded-none border-x-0">
		{#each $projects as project}
			<ProjectItem bind:item={project} />
		{/each}
	</Listgroup>

	<NewModal bind:open={openNewModal} bind:formData={data.form} on:close={handleCloseModal} />
</section>
