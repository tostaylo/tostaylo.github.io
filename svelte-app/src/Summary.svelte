<script lang="ts">
  import type { PostSummary } from "./types/types";
  export let postSummary: PostSummary;
</script>

<style>
  article {
    position: relative;
    padding: 0 0 0 2em;
    border-bottom: 1px solid #eee;
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

<a href="#/item/{postSummary.id}">
  <article>
    <span>{postSummary.id}</span>
    <h2>{postSummary.title}</h2>
    <p class="meta">By {postSummary.user}</p>
  </article>
</a>
