import { writable, Writable } from 'svelte/store';
import type { Book } from './Book';

export const booksStore: Writable<Book[]> = writable([]);
