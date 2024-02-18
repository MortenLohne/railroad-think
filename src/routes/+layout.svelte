<script>
import '/src/assets/profile.css';
import '/src/assets/global.css';
import '/src/assets/variables.css.ts';

const routes = Object.keys(import.meta.glob('./*/+page.svelte')).map(str =>
  str.replace('./', '').replace('/+page.svelte', '')
);
</script>

<nav>
  <ul>
    {#each routes as route}
      <li>
        <a href="./{route}">{route}</a>
      </li>
    {/each}
  </ul>
</nav>

<main>
  <slot />
</main>

<style>
nav {
  background-color: var(--blue-shade-1);
}

ul {
  padding: 0;
  display: flex;
  list-style-type: none;
}

a {
  color: white;
  text-decoration: none;
  padding: var(--s-2) var(--s-2-5);
  display: block;
  font-weight: 450;
}

li:first-of-type a {
  padding-left: var(--s-3-5);
}

a:focus-visible,
a:hover {
  background-color: var(--blue-shade-3);
  color: var(--blue-tint-3);
}

a:focus-visible {
  outline: 2px solid var(--blue-tint-1);
}

main {
  background-color: var(--blue-tint-4);
  height: 100%;
  padding: var(--s-2);
  flex-grow: 1;
}

:global(body),
:global(html) {
  min-height: 100%;
}

:global(.railroad-think) {
  display: flex;
  flex-direction: column;
  height: 100%;
}
</style>
