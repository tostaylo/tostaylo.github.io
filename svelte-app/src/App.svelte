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
        const loadingPlaceholder = document.getElementById(
          "loading-placeholder"
        );

        if (loadingPlaceholder) {
          loadingPlaceholder.remove();
        }

        postSummaries = data;
      });
  }

  async function getPost(event: any) {
    const { id } = event.detail;
    const data = await fetch(`${apiPre}posts/html/post.${id}.html`).then((r) =>
      r.text()
    );

    post = {
      ...postSummaries[id - 1],
      html_content: data,
    };

    isListPage = false;
  }

  function returnToList() {
    isListPage = true;
  }

  onMount(async () => {
    getPosts();
  });
</script>

<style>
  main {
    position: relative;
    max-width: 800px;
    margin: 0 auto;
    padding: 1em;
  }

  main :global(.meta) {
    color: #999;
    font-size: 12px;
    margin: 0 0 1em 0;
  }
</style>

<main>
  {#if !isListPage}
    <Post {post} on:returnToList={returnToList} />
  {:else if postSummaries && isListPage}
    <List on:getPost={getPost} {postSummaries} />
  {/if}
</main>
