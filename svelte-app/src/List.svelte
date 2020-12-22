<script lang="ts">
  import Summary from "./Summary.svelte";
  import type { PostSummary } from "./types/types";
  export let postSummaries: PostSummary[];
</script>

<style>
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
    <Summary on:getPost {postSummary} />
  {/each}
{:else}
  <p class="loading">loading...</p>
{/if}
