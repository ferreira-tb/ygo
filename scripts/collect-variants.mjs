import { writeFile } from 'node:fs/promises';

const response = await fetch('https://db.ygoprodeck.com/api/v7/cardinfo.php');
const cards = await response.json().then((it) => it.data);

let variants = {
  attributes: new Set(),
  frameTypes: new Set(),
  races: new Set(),
  types: new Set(),
};

for (const card of cards) {
  if (typeof card.attribute === 'string') {
    variants.attributes.add(card.attribute);
  }

  if (typeof card.frameType === 'string') {
    variants.frameTypes.add(card.frameType);
  }

  if (typeof card.race === 'string') {
    variants.races.add(card.race);
  }

  if (typeof card.type === 'string') {
    variants.types.add(card.type);
  }
}

variants = {
  attributes: Array.from(variants.attributes).sort(),
  frameTypes: Array.from(variants.frameTypes).sort(),
  races: Array.from(variants.races).sort(),
  types: Array.from(variants.types).sort(),
};

await writeFile(
  '.temp/variants.json',
  JSON.stringify(variants, null, 2),
  { encoding: 'utf-8' },
);
