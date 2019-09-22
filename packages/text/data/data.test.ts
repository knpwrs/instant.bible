import { sync as fg } from 'fast-glob';
import { basename } from 'path';
import { countBy, get, sum, values } from 'lodash';
import { DATA_DIR } from '../src/util';
import { TOTAL_VERSES, books } from '../src/meta';

const translations = fg(`${DATA_DIR}/*.json`);

const offsets = {
  NET: {
    Matthew: -3,
    Mark: -5,
    Luke: -2,
    John: -1,
    Acts: -4,
    Romans: -1,
    '2 Corinthians': -1,
  },
};

const nameMaps = {
  NET: {
    'Song of Solomon': 'The Song of Songs',
  },
};

translations.forEach(t => {
  const abbrev = basename(t, '.json');
  const data = require(t);

  test(`${abbrev} has correct number of total verses`, () => {
    const offset = offsets[abbrev] ? sum(values(offsets[abbrev])) : 0;
    expect(data.length).toBe(TOTAL_VERSES + offset);
  });

  const counts = countBy(data, 'book');

  books.forEach(b => {
    test(`${abbrev} has correct number of verses for book: ${b.name}`, () => {
      expect(counts[get(nameMaps, [abbrev, b.name], b.name)]).toBe(
        b.verses + get(offsets, [abbrev, b.name], 0),
      );
    });
  });
});
