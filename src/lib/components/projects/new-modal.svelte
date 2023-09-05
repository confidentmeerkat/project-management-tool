<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	import { ButtonGroup, InputAddon, Label, Modal, Input, Heading, Button } from 'flowbite-svelte';

	import { open as openPicker } from '@tauri-apps/api/dialog';

	import { sentenceCase } from 'change-case';
	import projects from '$lib/store/projects';
	import { createProject, getProjects } from '$lib/api/tauri/projects';

	// Props
	export let open = false;

	let path = '';
	let title = '';

	const dispatch = createEventDispatcher();

	const openDirectoryPicker = async () => {
		const p = (await openPicker({ directory: true, multiple: false })) as string;
		path = p;
		title = sentenceCase(p.split('\\').reverse()[0]);
	};

	const handleSubmit = async () => {
		const result = await createProject({ title, path });
		if (result === 'success') {
			projects.set(await getProjects());
			dispatch('close');
		} else {
			console.log('Failed');
		}
	};
</script>

<Modal bind:open on:close>
	<form method="POST" on:submit|stopPropagation|preventDefault={handleSubmit}>
		<Heading tag="h4">Add a Project</Heading>

		<div class="mt-4 space-y-1">
			<Label for="path">Path</Label>
			<ButtonGroup class="w-full">
				<InputAddon size="sm">
					<button type="button" class="w-20" on:click={openDirectoryPicker}>Choose Path</button>
				</InputAddon>
				<Input id="path" name="path" bind:value={path} required />
			</ButtonGroup>
		</div>

		<div class="mt-4 space-y-1">
			<Label for="path">Title</Label>
			<Input id="title" name="title" bind:value={title} required />
		</div>

		<Button type="submit" class="mt-4">Submit</Button>
	</form>
</Modal>
