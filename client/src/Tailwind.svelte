<style global style lang="postcss">
  @tailwind base;
  @tailwind components;

  @tailwind utilities;

  *:where(:not(iframe, canvas, img, svg, video):not(svg *, symbol *)) {
      all: unset;
      display: revert;
  }

  *,
  *::before,
  *::after {
      box-sizing: border-box;
  }

  body {
    height: 100%;
  }

  ol, ul {
      list-style: none;
  }

  img {
      max-width: 100%;
  }

  table {
      border-collapse: collapse;
  }

  textarea {
      white-space: revert;
  }

  :where([contenteditable]){
      -webkit-user-modify: read-write;
      overflow-wrap: break-word;
      -webkit-line-break: after-white-space;
  }
</style>
