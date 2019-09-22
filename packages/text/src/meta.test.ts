import { books } from './meta';

test('meta info consistent', () => {
  const otCount = 39;
  const ntCount = 27;
  expect(books.length).toBe(otCount + ntCount);
  books.forEach(book => {
    expect(book.verseCounts.length).toBe(book.chapters);
    expect(book.verseCounts.reduce((s, c) => s + c, 0)).toBe(book.verses);
  });
  expect(books.reduce((s, b) => s + b.verses, 0)).toBe(31102);
});
