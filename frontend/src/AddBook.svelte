<script lang="ts">
    import { booksStore } from "./stores";
    import type { Environment } from './Environment';
    import type { Book } from "./Book";

    const postBook = async (book: Book): Promise<Book> => {
        return await fetch(`${hostname}:${port}/books`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(book),
        }).then((response) => response.json());
    };

    let title = "";
    let author = "";

    let { BOOKS_HOST: hostname, BOOKS_PORT: port } = process.env as Environment;
    const onSubmit = async () => {
        const newBook = await postBook({
            title,
            author,
        });
        title = "";
        author = "";
        booksStore.update((books) => [...books, newBook]);
    };
</script>

<form on:submit|preventDefault={onSubmit}>
    <label for="title">Title</label>
    <input id="title" type="text" bind:value={title} />
    <label for="author">Author</label>
    <input id="author" type="text" bind:value={author} />
    <button type="submit">Add</button>
</form>
