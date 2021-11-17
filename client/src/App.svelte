<script lang="ts">
  import Tailwind from './Tailwind.svelte'
</script>

<Tailwind />

<div id="app">Hello world!</div>

<style>
  :global(*) {
    padding: 0;
    margin: 0;
    box-sizing: border-box;
    line-height: 1.5;
  }

  :global(body) {
    height: 100vh;
    font-family: 'Lato', sans-serif;
    font-size: medium;
  }

  :global(h1, h2, h3, h4, h5, h6) {
    font-family: 'Roboto', sans-serif;
  }
</style>
