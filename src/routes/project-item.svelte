<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { Button, Heading, ListgroupItem, P } from 'flowbite-svelte';
	import type { Project } from '../types';

	export let item: Project;

	const handleOpen = async () => {
		invoke('open_project', { path: item.path.replaceAll('\\', '/') }).then((data) =>
			console.log(data)
		);
	};
</script>

<ListgroupItem normalClass="flex-row flex">
	<div class="flex flex-1 flex-col">
		<Heading tag="h6" customSize="text-base font-semibold">
			{item.path.split('/').reverse()[0]}
		</Heading>

		<P size="sm">{item.path}</P>
	</div>

	<div class="flex items-center">
		<Button outline color="dark" class="!py-1 h-fit" on:click={handleOpen} size="xs">Open</Button>
	</div>
</ListgroupItem>
