import { writable } from 'svelte/store';
import type { Project } from '../../types';

const projects = writable<Project[]>([]);

export default projects;
