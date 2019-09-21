import { books } from './meta';

test('meta info consistent', () => {
  expect(books.length).toBe(39);
  books.forEach(book => {
    expect(book.verseCounts.length).toBe(book.chapters);
    expect(book.verseCounts.reduce((s, c) => s + c, 0)).toBe(book.verses);
  });
});
