import type { Profile } from '$generated/Profile';
import { writable } from 'svelte/store';

export const ProfileStore = writable<Profile[]>([]);
