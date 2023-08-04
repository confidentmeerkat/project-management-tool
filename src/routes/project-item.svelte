<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import type { Project } from '../types';

	export let item: Project;

	const handleOpen = async () => {
		invoke('open_project', { path: item.path.replaceAll('\\', '/') }).then((data) =>
			console.log(data)
		);
	};
</script>

<li class="rounded-md border border-gray-400 p-4 flex flex-row w-full">
	<div class="flex flex-1 flex-col">
		<h6>{item.path.split('/').reverse()[0]}</h6>
		<p>{item.path}</p>
	</div>

	<button on:click={handleOpen}>Open</button>
</li>
