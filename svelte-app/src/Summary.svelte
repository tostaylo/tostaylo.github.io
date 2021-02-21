<script lang="ts">
	import type { PostSummary } from './types/types';
	import { createEventDispatcher } from 'svelte';
	export let postSummary: PostSummary;
	const dispatch = createEventDispatcher();

	function getPost(id: number) {
		dispatch('getPost', {
			id,
		});
	}
</script>

<article>
	<span>{postSummary.id}</span>
	<h2 on:click={() => getPost(postSummary.id)}>{postSummary.title}</h2>
	<p class="meta">
		By
		{postSummary.user}
		on
		{postSummary.date}
		{#if postSummary.url}
			<a href={postSummary.url}>View on Dev.to</a>
		{/if}
	</p>
</article>

<style>
	article {
		position: relative;
		padding: 0 0 0 2em;
		border-bottom: 1px solid var(--text-color-mod);
		cursor: pointer;
	}
	h2 {
		font-size: 1em;
		margin: 0.5em 0;
	}
	span {
		position: absolute;
		left: 0;
	}
</style>
