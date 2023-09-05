import { invoke } from '@tauri-apps/api/tauri';
import type { Project } from '../../../types';

export const getProjects = async () => {
	return await invoke<Project[]>('get_projects');
};

export const createProject = async (project: Project) => {
	return invoke('create_project', project);
};
