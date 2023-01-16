<script>
import { dev } from '$app/env';
import Section from '/src/lib/Section.svelte';
export const prerender = true;

// export let target; // Element the app attaches to
let message = 'Hei, verden!';
</script>

<article class:dev>
  <Section>
    <h1>{message}</h1>
  </Section>
</article>

<style>
article.dev {
  padding: var(--s-3);
}

h1 {
  color: var(--black);
}
</style>
