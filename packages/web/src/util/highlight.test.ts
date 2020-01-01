import highlight from './highlight';

const cases: Array<[string, Array<string>, ReturnType<typeof highlight>]> = [
  [
    'Hello, World!',
    ['hello'],
    [
      { text: 'Hello', highlight: true },
      { text: ', World!', highlight: false },
    ],
  ],
  [
    'Hello, World!',
    ['hello', 'world'],
    [
      { text: 'Hello', highlight: true },
      { text: ', ', highlight: false },
      { text: 'World', highlight: true },
      { text: '!', highlight: false },
    ],
  ],
  [
    `“Look,” said Esau, “I’m about to die! What use is the birthright to me?”`,
    ['i'],
    [
      { text: '“Look,” said Esau, “', highlight: false },
      { text: 'I', highlight: true },
      {
        text: '’m about to die! What use ',
        highlight: false,
      },
      {
        text: 'i',
        highlight: true,
      },
      {
        text: 's the birthright to me?”',
        highlight: false,
      },
    ],
  ],
];

cases.forEach(([text, words, result]) => {
  test(`highlights "${text}" with ["${words.join('", "')}"]`, () => {
    expect(highlight(text, words)).toEqual(result);
  });
});
