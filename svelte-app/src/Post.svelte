<script lang="ts">
  import type { PostType } from "./types/types";
  export let post: PostType;
  export let returnTo: string;
</script>

<style>
  article {
    margin: 0 0 1em 0;
  }

  .a {
    display: block;
    margin: 0 0 1em 0;
  }

  h1 {
    font-size: 1.4em;
    margin: 0;
  }
</style>

<a class="a" href={returnTo}>&laquo; back</a>

<article>
  <h1>{post.title}</h1>

  <p class="meta">submitted by {post.user} {post.time_ago}</p>
  <p>
    {@html post.html_content}
  </p>
</article>
