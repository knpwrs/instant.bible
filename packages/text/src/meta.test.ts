import { TOTAL_VERSES, TOTAL_BOOKS, books } from './meta';

test('meta info consistent', () => {
  expect(books.length).toBe(TOTAL_BOOKS);
  books.forEach(book => {
    expect(book.verseCounts.length).toBe(book.chapters);
    expect(book.verseCounts.reduce((s, c) => s + c, 0)).toBe(book.verses);
  });
  expect(books.reduce((s, b) => s + b.verses, 0)).toBe(TOTAL_VERSES);
});
