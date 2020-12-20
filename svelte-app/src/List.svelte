<script lang="ts">
  import Summary from "./Summary.svelte";
  import type { PostSummary } from "./types/types";

  let postSummaries: PostSummary[];

  $: fetch("/posts.json")
    .then((r) => r.json())
    .then((data) => {
      postSummaries = data;
    });
</script>

<style>
  a {
    padding: 2em;
    display: block;
  }

  .loading {
    opacity: 0;
    animation: 0.4s 0.8s forwards fade-in;
  }

  @keyframes fade-in {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }
</style>

{#if postSummaries}
  {#each postSummaries as postSummary}
    <Summary {postSummary} />
  {/each}
{:else}
  <p class="loading">loading...</p>
{/if}
