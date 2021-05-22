<script>
    import Book from './Book.svelte';
    let hostname = process.env.BOOKS_HOST;
    let port = process.env.BOOKS_PORT;
    const fetchBooks = (async () => {
        console.log(hostname, port);
        const response = await fetch(`${hostname}:${port}/books`);
        return await response.json();
    })();
</script>

{#await fetchBooks}
	<p>...waiting</p>
{:then books}
    {#each books as book}
        <Book book={book}/>
    {/each}
{:catch error}
	<p>An error occurred!</p>
{/await}

