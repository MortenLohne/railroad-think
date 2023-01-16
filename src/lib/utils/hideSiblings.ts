import isPaywalled from './isPaywalled';

/**
 * @param {HTMLElement} target - The element this app mounts to.
 */
export default function hideSiblings(target: HTMLElement) {
  const paywall = isPaywalled();
  if (target?.parentElement?.parentElement) {
    const siblings = [...target.parentElement.parentElement.children];
    const isSelf = n => n === target.parentElement;
    const isLogin = n => /logg inn/i.test(n.innerText);

    siblings
      .filter(e => e instanceof HTMLElement)
      .map(e => <HTMLElement>e)
      .filter(n => !isSelf(n) && !isLogin(n))
      .forEach(sibling => (sibling.style.display = 'none'));

    if (paywall) {
      const isDiv = n => n.tagName === 'DIV';
      const isHeading = n => n.tagName === 'H1';
      const isSalesposter = n => n.firstElementChild?.tagName === 'IFRAME';

      siblings
        .filter(e => e instanceof HTMLElement)
        .map(e => <HTMLElement>e)
        .filter(
          n =>
            (isHeading(n) || isDiv(n)) &&
            !(isLogin(n) || isSalesposter(n) || isSelf(n))
        )
        .forEach(sibling => (sibling.style.display = 'none'));
    }
  }
}
