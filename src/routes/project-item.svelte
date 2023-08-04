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

<div>
	<h6>{item.path.split('/').reverse()[0]}</h6>
	<p>{item.path}</p>
	<button on:click={handleOpen}>Open</button>
</div>
