<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	import { ButtonGroup, InputAddon, Label, Modal, Input, Heading, Button } from 'flowbite-svelte';
	import { superForm } from 'sveltekit-superforms/client';

	import { open as openPicker } from '@tauri-apps/api/dialog';

	import { sentenceCase } from 'change-case';
	import projects from '$lib/store/projects';
	import { createProject, getProjects } from '$lib/api/tauri/projects';
	import type { Project } from '../../../types';

	// Props
	export let open = false;
	export let formData;

	const dispatch = createEventDispatcher();

	const openDirectoryPicker = async () => {
		const path = (await openPicker({ directory: true, multiple: false })) as string;
		$form.path = path;
		$form.title = sentenceCase(path.split('\\').reverse()[0]);
	};

	const { form, enhance, constraints } = superForm(formData, {
		async onResult(e) {
			console.log('e:', e);
			if (e.result.type === 'success') {
				await createProject($form as Project);
				projects.set(await getProjects());
				dispatch('close');
			}
		}
	});
</script>

<Modal bind:open on:close>
	<form method="POST" use:enhance>
		<Heading tag="h4">Add a Project</Heading>

		<div class="mt-4 space-y-1">
			<Label for="path">Path</Label>
			<ButtonGroup class="w-full">
				<InputAddon size="sm">
					<button type="button" class="w-20" on:click={openDirectoryPicker}>Choose Path</button>
				</InputAddon>
				<Input id="path" name="path" bind:value={$form.path} {...$constraints.path} />
			</ButtonGroup>
		</div>

		<div class="mt-4 space-y-1">
			<Label for="path">Title</Label>
			<Input id="title" name="title" bind:value={$form.title} {...$constraints.title} />
		</div>

		<Button type="submit" class="mt-4">Submit</Button>
	</form>
</Modal>
