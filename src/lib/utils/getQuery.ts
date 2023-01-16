// https://stackoverflow.com/questions/2090551/parse-query-string-in-javascript

type Query = { [key: string]: string | boolean };

function parseQuery(queryString: string): Query {
  let query = {};
  let pairs = queryString.split('&');
  for (let i = 0; i < pairs.length; i++) {
    let pair = pairs[i].split('=');
    query[decodeURIComponent(pair[0])] =
      decodeURIComponent(pair[1] || '') || true;
  }
  if (query['data']) query['data'] = decodeURIComponent(query['data']);
  return query;
}

export default function getQuery(target: HTMLElement): Query {
  if (!target) return null;
  let parent = target.parentElement;

  let url = parent.hasAttribute('data-origin')
    ? parent.getAttribute('data-origin')
    : window.location.search;
  if (!url) return null;

  let query = url.split('?');
  if (query.length < 2) return null;

  return parseQuery(query[1]);
}
