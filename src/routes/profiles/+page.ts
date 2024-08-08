import { tauri } from '@tauri-apps/api';
import type { PageLoad } from './$types';
import Commands from '$lib/commands';
import type { Profile } from '$shared_types/generated/Profile';

export const load = (async () => {
    const profiles = await tauri.invoke<Profile[]>(Commands.FetchAllProfiles)
    return { profiles };
}) satisfies PageLoad;