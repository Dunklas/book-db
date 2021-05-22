<script lang="ts">
	import Books from "./Books.svelte";
	import AddBook from "./AddBook.svelte";
</script>

<header>
	<h1>My books</h1>
</header>
<main>
	<AddBook />
	<Books />
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
		text-align: center;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
