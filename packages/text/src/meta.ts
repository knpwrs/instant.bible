import * as proto from './proto';

const { Book } = proto.instantbible.data;

export const TOTAL_VERSES = 31102;
export const TOTAL_OT_BOOKS = 39;
export const TOTAL_NT_BOOKS = 27;
export const TOTAL_BOOKS = TOTAL_OT_BOOKS + TOTAL_NT_BOOKS;

export type BookMeta = {
  readonly name: string;
  readonly chapters: number;
  readonly verses: number;
  readonly proto: proto.instantbible.data.Book;
};

// For the following data the book names are the common names while the numbers are from the KJV
export const books: BookMeta[] = [
  // Law
  {
    name: 'Genesis',
    chapters: 50,
    verses: 1533,
    proto: Book.GENESIS,
  },
  {
    name: 'Exodus',
    chapters: 40,
    verses: 1213,
    proto: Book.EXODUS,
  },
  {
    name: 'Leviticus',
    chapters: 27,
    verses: 859,
    proto: Book.LEVITICUS,
  },
  {
    name: 'Numbers',
    chapters: 36,
    verses: 1288,
    proto: Book.NUMBERS,
  },
  {
    name: 'Deuteronomy',
    chapters: 34,
    verses: 959,
    proto: Book.DEUTERONOMY,
  },
  // Old Testament Narrative
  {
    name: 'Joshua',
    chapters: 24,
    verses: 658,
    proto: Book.JOSHUA,
  },
  {
    name: 'Judges',
    chapters: 21,
    verses: 618,
    proto: Book.JUDGES,
  },
  {
    name: 'Ruth',
    chapters: 4,
    verses: 85,
    proto: Book.RUTH,
  },
  {
    name: '1 Samuel',
    chapters: 31,
    verses: 810,
    proto: Book.FIRST_SAMUEL,
  },
  {
    name: '2 Samuel',
    chapters: 24,
    verses: 695,
    proto: Book.SECOND_SAMUEL,
  },
  {
    name: '1 Kings',
    chapters: 22,
    verses: 816,
    proto: Book.FIRST_KINGS,
  },
  {
    name: '2 Kings',
    chapters: 25,
    verses: 719,
    proto: Book.SECOND_KINGS,
  },
  {
    name: '1 Chronicles',
    chapters: 29,
    verses: 942,
    proto: Book.FIRST_CHRONICLES,
  },
  {
    name: '2 Chronicles',
    chapters: 36,
    verses: 822,
    proto: Book.SECOND_CHRONICLES,
  },
  {
    name: 'Ezra',
    chapters: 10,
    verses: 280,
    proto: Book.EZRA,
  },
  {
    name: 'Nehemiah',
    chapters: 13,
    verses: 406,
    proto: Book.EZRA,
  },
  {
    name: 'Esther',
    chapters: 10,
    verses: 167,
    proto: Book.ESTHER,
  },
  // Wisdom Literature
  {
    name: 'Job',
    chapters: 42,
    verses: 1070,
    proto: Book.JOB,
  },
  {
    name: 'Psalms',
    chapters: 150,
    verses: 2461,
    proto: Book.PSALMS,
  },
  {
    name: 'Proverbs',
    chapters: 31,
    verses: 915,
    proto: Book.PROVERBS,
  },
  {
    name: 'Ecclesiastes',
    chapters: 12,
    verses: 222,
    proto: Book.ECCLESIASTES,
  },
  {
    name: 'Song of Solomon',
    chapters: 8,
    verses: 117,
    proto: Book.SONG_OF_SOLOMON,
  },
  {
    name: 'Isaiah',
    chapters: 66,
    verses: 1292,
    proto: Book.ISAIAH,
  },
  {
    name: 'Jeremiah',
    chapters: 52,
    verses: 1364,
    proto: Book.JEREMIAH,
  },
  {
    name: 'Lamentations',
    chapters: 5,
    verses: 154,
    proto: Book.LAMENTATIONS,
  },
  {
    name: 'Ezekiel',
    chapters: 48,
    verses: 1273,
    proto: Book.EZEKIEL,
  },
  {
    name: 'Daniel',
    chapters: 12,
    verses: 357,
    proto: Book.DANIEL,
  },
  // Minor Prophets
  {
    name: 'Hosea',
    chapters: 14,
    verses: 197,
    proto: Book.HOSEA,
  },
  {
    name: 'Joel',
    chapters: 3,
    verses: 73,
    proto: Book.JOEL,
  },
  {
    name: 'Amos',
    chapters: 9,
    verses: 146,
    proto: Book.AMOS,
  },
  {
    name: 'Obadiah',
    chapters: 1,
    verses: 21,
    proto: Book.OBADIAH,
  },
  {
    name: 'Jonah',
    chapters: 4,
    verses: 48,
    proto: Book.JONAH,
  },
  {
    name: 'Micah',
    chapters: 7,
    verses: 105,
    proto: Book.MICAH,
  },
  {
    name: 'Nahum',
    chapters: 3,
    verses: 47,
    proto: Book.NAHUM,
  },
  {
    name: 'Habakkuk',
    chapters: 3,
    verses: 56,
    proto: Book.HABAKKUK,
  },
  {
    name: 'Zephaniah',
    chapters: 3,
    verses: 53,
    proto: Book.ZEPHANIAH,
  },
  {
    name: 'Haggai',
    chapters: 2,
    verses: 38,
    proto: Book.HAGGAI,
  },
  {
    name: 'Zechariah',
    chapters: 14,
    verses: 211,
    proto: Book.ZECHARIAH,
  },
  {
    name: 'Malachi',
    chapters: 4,
    verses: 55,
    proto: Book.MALACHI,
  },
  // New Testament Narrative
  {
    name: 'Matthew',
    chapters: 28,
    verses: 1071,
    proto: Book.MATTHEW,
  },
  {
    name: 'Mark',
    chapters: 16,
    verses: 678,
    proto: Book.MARK,
  },
  {
    name: 'Luke',
    chapters: 24,
    verses: 1151,
    proto: Book.LUKE,
  },
  {
    name: 'John',
    chapters: 21,
    verses: 879,
    proto: Book.JOHN,
  },
  {
    name: 'Acts',
    chapters: 28,
    verses: 1007,
    proto: Book.ACTS,
  },
  // Pauline Epistles
  {
    name: 'Romans',
    chapters: 16,
    verses: 433,
    proto: Book.ROMANS,
  },
  {
    name: '1 Corinthians',
    chapters: 16,
    verses: 437,
    proto: Book.FIRST_CORINTHIANS,
  },
  {
    name: '2 Corinthians',
    chapters: 13,
    verses: 257,
    proto: Book.SECOND_CORINTHIANS,
  },
  {
    name: 'Galatians',
    chapters: 6,
    verses: 149,
    proto: Book.GALATIANS,
  },
  {
    name: 'Ephesians',
    chapters: 6,
    verses: 155,
    proto: Book.EPHESIANS,
  },
  {
    name: 'Philippians',
    chapters: 4,
    verses: 104,
    proto: Book.PHILIPPIANS,
  },
  {
    name: 'Colossians',
    chapters: 4,
    verses: 95,
    proto: Book.COLOSSIANS,
  },
  {
    name: '1 Thessalonians',
    chapters: 5,
    verses: 89,
    proto: Book.FIRST_THESSALONIANS,
  },
  {
    name: '2 Thessalonians',
    chapters: 3,
    verses: 47,
    proto: Book.SECOND_THESSALONIANS,
  },
  {
    name: '1 Timothy',
    chapters: 6,
    verses: 113,
    proto: Book.FIRST_TIMOTHY,
  },
  {
    name: '2 Timothy',
    chapters: 4,
    verses: 83,
    proto: Book.SECOND_TIMOTHY,
  },
  {
    name: 'Titus',
    chapters: 3,
    verses: 46,
    proto: Book.TITUS,
  },
  {
    name: 'Philemon',
    chapters: 1,
    verses: 25,
    proto: Book.PHILEMON,
  },
  // General Epistles
  {
    name: 'Hebrews',
    chapters: 13,
    verses: 303,
    proto: Book.HEBREWS,
  },
  {
    name: 'James',
    chapters: 5,
    verses: 108,
    proto: Book.JAMES,
  },
  {
    name: '1 Peter',
    chapters: 5,
    verses: 105,
    proto: Book.FIRST_PETER,
  },
  {
    name: '2 Peter',
    chapters: 3,
    verses: 61,
    proto: Book.SECOND_PETER,
  },
  {
    name: '1 John',
    chapters: 5,
    verses: 105,
    proto: Book.FIRST_JOHN,
  },
  {
    name: '2 John',
    chapters: 1,
    verses: 13,
    proto: Book.SECOND_JOHN,
  },
  {
    name: '3 John',
    chapters: 1,
    verses: 14,
    proto: Book.THIRD_JOHN,
  },
  {
    name: 'Jude',
    chapters: 1,
    verses: 25,
    proto: Book.JUDE,
  },
  // Apocalyptic Epistle
  {
    name: 'Revelation',
    chapters: 22,
    verses: 404,
    proto: Book.REVELATION,
  },
];
