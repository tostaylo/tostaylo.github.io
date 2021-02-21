<script lang="ts">
	import type { PostType } from './types/types';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();
	export let post: PostType;
</script>

<button class="a" on:click={() => dispatch('returnToList')}>&laquo; back</button>

<article>
	<h1>{post.title}</h1>

	<p class="meta">
		by
		{post.user}
		{post.date}
		{#if post.url}
			<a href={post.url}>View on Dev.to</a>
		{/if}
	</p>
	<p>
		{@html post.html_content}
	</p>
</article>

<style>
	article {
		margin: 0 0 1em 0;
	}

	.a {
		display: block;
		margin: 0 0 1em 0;
		cursor: pointer;
		border: none;
		background-color: transparent;
		color: var(--text-color-mod);
	}

	h1 {
		font-size: 1.4em;
		margin: 0;
	}
</style>
