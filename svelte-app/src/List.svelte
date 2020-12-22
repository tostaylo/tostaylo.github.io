<script lang="ts">
  import Summary from "./Summary.svelte";
  import type { PostSummary } from "./types/types";
  export let postSummaries: PostSummary[];
</script>

<style>
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
{/if}
