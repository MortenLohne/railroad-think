import pieces from '../data/pieces.csv';

const all = pieces.map(d => d.name);
const codes = Object.fromEntries(pieces.map(d => [d.name, d.code]));
const names = Object.fromEntries(pieces.map(d => [d.code, d.name]));
const rollable = pieces.map(d => d.name).filter(name => name[0] !== 'X');
const placeable = pieces.map(d => d.name).filter(name => name[0] === 'X');

export { placeable, rollable, codes, names };
export default all;
