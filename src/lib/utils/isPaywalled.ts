/**
 * @returns {boolean} Whether the article is currently behind a paywall.
 */
export default function isPaywalled(): boolean {
  if (window.location.search.includes('salesposter')) return true; // For easier testing
  // Hacky ...
  const test1 =
    window['__PRELOADED_STATE__']?.page?.compositions?.[0] &&
    JSON.stringify(window['__PRELOADED_STATE__'].page.compositions[0]).includes(
      'salesposter'
    );

  // @ts-ignore
  const test2 =
    window['__PRELOADED_STATE__']?.location?.pathname?.includes?.(
      'salesposter'
    );
  // ... very hacky.
  return test1 || test2;
}
