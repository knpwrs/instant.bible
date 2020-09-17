import { flatMap } from 'lodash';

const highlight = (
  text: string,
  words: Array<string>,
): Array<{ highlight: boolean; text: string }> => {
  const chunks = words.reduce(
    (chunks, word) => {
      const regex = new RegExp(`(\\b${word})`, 'gi');

      return flatMap(chunks, (chunk) =>
        chunk.highlight
          ? chunk
          : chunk.text
              .split(regex)
              .filter(Boolean)
              .map((text) => ({ text, highlight: regex.test(text) })),
      );
    },
    [{ highlight: false, text }] as ReturnType<typeof highlight>,
  );

  return chunks;
};

export default highlight;
