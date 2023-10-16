import { writable, get } from 'svelte/store'

export type User = {
	id: number
	username: string
}

export const user = writable<User | undefined>();
export const ready = writable<boolean>(false);

export function waitForReady(): Promise<void> {
	if (get(ready)) return Promise.resolve();

	return new Promise((resolve) => {
		const unsubscribe = ready.subscribe((value) => {
			if (value) {
				unsubscribe();
				resolve();
			}
		});
	});
}

export function isLoggedIn(): Promise<boolean> {
	return waitForReady().then(() => get(user) !== undefined);
}
