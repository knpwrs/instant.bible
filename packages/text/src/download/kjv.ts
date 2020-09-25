import got from 'got';
import shaFn = require('crypto-js/sha256');
import { isNumber } from 'lodash';
import base, { Downloader } from './base';
import * as proto from '../proto';

const { Book, VerseText } = proto.instantbible.data;

const URL = 'https://www.gutenberg.org/cache/epub/10/pg10.txt';
const SHA256 =
  '54fa639b823866f7dd3c79aec791a19010a89de7e3ce06d4b24cf7530d9b1d0e';

const nameMap = {
  'The First Book of Moses:  Called Genesis': Book.GENESIS,
  'The Second Book of Moses:  Called Exodus': Book.EXODUS,
  'The Third Book of Moses:  Called Leviticus': Book.LEVITICUS,
  'The Fourth Book of Moses:  Called Numbers': Book.NUMBERS,
  'The Fifth Book of Moses:  Called Deuteronomy': Book.DEUTERONOMY,
  'The Book of Joshua': Book.JOSHUA,
  'The Book of Judges': Book.JUDGES,
  'The Book of Ruth': Book.RUTH,
  'The First Book of Samuel': Book.FIRST_SAMUEL,
  'The Second Book of Samuel': Book.SECOND_SAMUEL,
  'The Third Book of the Kings': Book.FIRST_KINGS,
  'The Fourth Book of the Kings': Book.SECOND_KINGS,
  'The First Book of the Chronicles': Book.FIRST_CHRONICLES,
  'The Second Book of the Chronicles': Book.SECOND_CHRONICLES,
  Ezra: Book.EZRA,
  'The Book of Nehemiah': Book.NEHEMIAH,
  'The Book of Esther': Book.ESTHER,
  'The Book of Job': Book.JOB,
  'The Book of Psalms': Book.PSALMS,
  'The Proverbs': Book.PROVERBS,
  Ecclesiastes: Book.ECCLESIASTES,
  'The Song of Solomon': Book.SONG_OF_SOLOMON,
  'The Book of the Prophet Isaiah': Book.ISAIAH,
  'The Book of the Prophet Jeremiah': Book.JEREMIAH,
  'The Lamentations of Jeremiah': Book.LAMENTATIONS,
  'The Book of the Prophet Ezekiel': Book.EZEKIEL,
  'The Book of Daniel': Book.DANIEL,
  Hosea: Book.HOSEA,
  Joel: Book.JOEL,
  Amos: Book.AMOS,
  Obadiah: Book.OBADIAH,
  Jonah: Book.JONAH,
  Micah: Book.MICAH,
  Nahum: Book.NAHUM,
  Habakkuk: Book.HABAKKUK,
  Zephaniah: Book.ZEPHANIAH,
  Haggai: Book.HAGGAI,
  Zechariah: Book.ZECHARIAH,
  Malachi: Book.MALACHI,
  'The Gospel According to Saint Matthew': Book.MATTHEW,
  'The Gospel According to Saint Mark': Book.MARK,
  'The Gospel According to Saint Luke': Book.LUKE,
  'The Gospel According to Saint John': Book.JOHN,
  'The Acts of the Apostles': Book.ACTS,
  'The Epistle of Paul the Apostle to the Romans': Book.ROMANS,
  'The First Epistle of Paul the Apostle to the Corinthians': Book.FIRST_CORINTHIANS,
  'The Second Epistle of Paul the Apostle to the Corinthians': Book.SECOND_CORINTHIANS,
  'The Epistle of Paul the Apostle to the Galatians': Book.GALATIANS,
  'The Epistle of Paul the Apostle to the Ephesians': Book.EPHESIANS,
  'The Epistle of Paul the Apostle to the Philippians': Book.PHILIPPIANS,
  'The Epistle of Paul the Apostle to the Colossians': Book.COLOSSIANS,
  'The First Epistle of Paul the Apostle to the Thessalonians':
    Book.FIRST_THESSALONIANS,
  'The Second Epistle of Paul the Apostle to the Thessalonians':
    Book.SECOND_THESSALONIANS,
  'The First Epistle of Paul the Apostle to Timothy': Book.FIRST_TIMOTHY,
  'The Second Epistle of Paul the Apostle to Timothy': Book.SECOND_TIMOTHY,
  'The Epistle of Paul the Apostle to Titus': Book.TITUS,
  'The Epistle of Paul the Apostle to Philemon': Book.PHILEMON,
  'The Epistle of Paul the Apostle to the Hebrews': Book.HEBREWS,
  'The General Epistle of James': Book.JAMES,
  'The First Epistle General of Peter': Book.FIRST_PETER,
  'The Second General Epistle of Peter': Book.SECOND_PETER,
  'The First Epistle General of John': Book.FIRST_JOHN,
  'The Second Epistle General of John': Book.SECOND_JOHN,
  'The Third Epistle General of John': Book.THIRD_JOHN,
  'The General Epistle of Jude': Book.JUDE,
  'The Revelation of Saint John the Devine': Book.REVELATION,
};

const startVerse = /^(\d+):(\d+)/;

const download: Downloader = async ({ d }) => {
  d('Downloading text');

  const rawText = await got(URL).text();
  const rawTextSha = shaFn(rawText).toString();

  if (rawTextSha !== SHA256) {
    throw new Error(
      `SHA of KJV text does not match! Expected ${SHA256} but got ${rawTextSha}`,
    );
  }

  const booksSeen = {};
  const lines = rawText
    .replace(/(?<=\d+:\d+)\r\n/g, ' ')
    .replace(/\s+(?=\d+:\d+)/g, '\n\n')
    .split(/\r?\n/);
  const verses: Array<proto.instantbible.data.VerseText> = [];

  {
    let book = -1;
    let chapter = 0;
    let verse = 0;
    let text = '';

    for (const line of lines) {
      if (isNumber(nameMap[line]) && !booksSeen[line]) {
        book = nameMap[line];
        booksSeen[line] = true;
      }
      if (book === -1) continue;
      if (startVerse.test(line)) {
        // New verse
        const [, sChapter, sVerse] = startVerse.exec(line);
        chapter = parseInt(sChapter, 10);
        verse = parseInt(sVerse, 10);
        text += line.replace(startVerse, '').trim();
      } else if (text && line) {
        text += ` ${line.trim()}`;
      } else if (text) {
        verses.push(new VerseText({
          key: { book, chapter, verse },
          text,
        }));
        chapter = 0;
        verse = 0;
        text = '';
      }
    }
  }

  return verses;
};

base('KJV', download);
