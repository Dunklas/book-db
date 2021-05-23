<script lang="ts">
    import { onMount } from "svelte";
    import Book from "./Book.svelte";
    import { booksStore } from "./stores";
    import type { Environment } from './Environment';
    import type { Book as IBook } from './Book';

    let { BOOKS_HOST: hostname, BOOKS_PORT: port } = process.env as Environment;
    onMount(async () => {
        const response = await fetch(`${hostname}:${port}/books`).then(
            (response) => response.json()
        );
        booksStore.update((books) => [...books, ...response]);
    });

    let books: IBook[];
    booksStore.subscribe((newBooks) => {
        books = newBooks;
    });
</script>

<section>
    {#each books as book}
        <Book {book} />
    {/each}
</section>
