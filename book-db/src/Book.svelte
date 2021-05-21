<script lang="ts">
    import type { Book } from './Book';
    export let book: Book; 
</script>

<div>
    <h3>{book.title}</h3>
    <p>{book.author}</p>
</div>
