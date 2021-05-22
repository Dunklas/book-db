<script>
    import { onMount } from "svelte";
    import Book from "./Book.svelte";
    import { booksStore } from "./stores";

    let hostname = process.env.BOOKS_HOST;
    let port = process.env.BOOKS_PORT;
    onMount(async () => {
        const response = await fetch(`${hostname}:${port}/books`).then(
            (response) => response.json()
        );
        booksStore.update((books) => [...books, ...response]);
    });

    let books;
    booksStore.subscribe((newBooks) => {
        books = newBooks;
    });
</script>

<section>
    {#each books as book}
        <Book {book} />
    {/each}
</section>
