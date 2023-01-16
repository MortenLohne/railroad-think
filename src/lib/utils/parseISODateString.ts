/**
 * @param {string | Date} input - Input string will be parsed. Input objects gets forwarded to the Date constructor.
 * @return {Date} This is the result
 */
function parseISODateString(input: string): Date {
  if (typeof input !== 'string' || !/^\d{4}-\d{2}-\d{2}/.test(input))
    return new Date(NaN);
  const string = input;
  // https://stackoverflow.com/a/42626876
  const date = <[number, number, number, number?, number?, number?]>string
    .split(/\D+/)
    .map(digit => parseInt(digit))
    .slice(0, 6);
  date[1] = date[1] - 1; // adjust month
  return new Date(...date);
}

export { parseISODateString };
