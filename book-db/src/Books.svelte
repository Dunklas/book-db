<script>
    import Book from './Book.svelte';
    let books = [
        {
            id: 1,
            title: 'House on the Borderland',
            author: 'William Hope Hodgson'
        },
        {
            id: 2,
            title: 'At the Mountain of Madness',
            author: 'H.P. Lovecraft'
        }
    ];
</script>

{#each books as book}
    <Book book={book}/>
{/each}
