<script>
// import Router, { link } from 'svelte-spa-router';
// import { wrap } from 'svelte-spa-router/wrap';

const glob_routes = import.meta.glob('./*.svelte');
const routes = Object.fromEntries(
  Object.entries(glob_routes).map(([key, importFunction]) => [
    key.replace(/(^\.|\.svelte$)/g, ''),
    // wrap({ asyncComponent: importFunction }),
  ])
);
console.log(routes);
</script>

<nav>
  <ul>
    {#each Object.keys(routes) as route}
      <li>
        <!-- <a href={route} use:link>{route.slice(1)}</a> -->
      </li>
    {/each}
  </ul>
</nav>

<main>
  <!-- <Router {routes} /> -->
</main>

<style>
nav {
  background-color: var(--blue-shade-1);
  margin-bottom: var(--s-2);
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

a:hover {
  background-color: var(--blue-shade-3);
  color: var(--blue-tint-3);
}

main {
  padding: var(--s-2);
}
</style>
