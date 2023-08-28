import { fail, type Actions } from '@sveltejs/kit';

import { superValidate } from 'sveltekit-superforms/client';
import { z } from 'zod';

import type { PageServerLoad } from './$types';
import { createProject } from '$lib/api/tauri/projects';

export const prerender = false;

const schema = z.object({
	path: z.string(),
	title: z.string()
});

export const load: PageServerLoad = async () => {
	const form = await superValidate(schema);

	return { form };
};

export const actions: Actions = {
	default: async ({ request }) => {
		const form = await superValidate(request, schema);
		console.log('POST', form);

		if (!form.valid) {
			// Again, always return { form } and things will just work.
			return fail(400, { form });
		}

		// await createProject(form.data);

		return { form };
	}
};
