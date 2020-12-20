<script lang="ts">
  import { onMount } from "svelte";
  import List from "./List.svelte";
  import Post from "./Post.svelte";
  import type { PostSummary, PostType } from "./types/types";

  export let apiPre: string;

  let postSummaries: PostSummary[];
  let isListPage: boolean = true;
  let post: PostType = {} as PostType;

  async function getPosts() {
    fetch(`${apiPre}posts/posts.json`)
      .then((r) => r.json())
      .then((data) => {
        postSummaries = data;
      });
  }

  async function hashchange() {
    // the poor man's router!
    const path = window.location.hash.slice(1);

    if (path.startsWith("/item")) {
      const id = path.slice(6);
      const idInt = +id;
      const data = await fetch(`${apiPre}posts/post.${id}.html`).then((r) =>
        r.text()
      );
      isListPage = false;

      post = {
        ...postSummaries[idInt - 1],
        html_content: data,
      };

      window.scrollTo(0, 0);
    } else if (path.startsWith("/posts")) {
      post = {} as PostType;
      isListPage = true;
    } else {
      window.location.hash = "/posts";
      isListPage = true;
    }
  }

  onMount(async () => {
    getPosts();
    hashchange();
  });
</script>

<style>
  main {
    position: relative;
    max-width: 800px;
    margin: 0 auto;
    min-height: 101vh;
    padding: 1em;
  }

  main :global(.meta) {
    color: #999;
    font-size: 12px;
    margin: 0 0 1em 0;
  }
</style>

<svelte:window on:hashchange={hashchange} />

<main>
  {#if post.id}
    <Post {post} returnTo="#/posts" />
  {:else if isListPage}
    <List {postSummaries} />
  {/if}
</main>
