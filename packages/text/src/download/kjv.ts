import * as got from 'got';
import shaFn = require('crypto-js/sha256');
import base, { Downloader } from './base';
import { Verse } from '../util';

const URL = 'https://www.gutenberg.org/cache/epub/10/pg10.txt';
const SHA256 =
  '54fa639b823866f7dd3c79aec791a19010a89de7e3ce06d4b24cf7530d9b1d0e';

const nameMap = {
  'The First Book of Moses:  Called Genesis': 'Genesis',
  'The Second Book of Moses:  Called Exodus': 'Exodus',
  'The Third Book of Moses:  Called Leviticus': 'Leviticus',
  'The Fourth Book of Moses:  Called Numbers': 'Numbers',
  'The Fifth Book of Moses:  Called Deuteronomy': 'Deuteronomy',
  'The Book of Joshua': 'Joshua',
  'The Book of Judges': 'Judges',
  'The Book of Ruth': 'Ruth',
  'The First Book of Samuel': '1 Samuel',
  'The Second Book of Samuel': '2 Samuel',
  'The Third Book of the Kings': '1 Kings',
  'The Fourth Book of the Kings': '2 Kings',
  'The First Book of the Chronicles': '1 Chronicles',
  'The Second Book of the Chronicles': '2 Chronicles',
  Ezra: 'Ezra',
  'The Book of Nehemiah': 'Nehemiah',
  'The Book of Esther': 'Esther',
  'The Book of Job': 'Job',
  'The Book of Psalms': 'Psalms',
  'The Proverbs': 'Proverbs',
  Ecclesiastes: 'Ecclesiastes',
  'The Song of Solomon': 'Song of Solomon',
  'The Book of the Prophet Isaiah': 'Isaiah',
  'The Book of the Prophet Jeremiah': 'Jeremiah',
  'The Lamentations of Jeremiah': 'Lamentations',
  'The Book of the Prophet Ezekiel': 'Ezekiel',
  'The Book of Daniel': 'Daniel',
  Hosea: 'Hosea',
  Joel: 'Joel',
  Amos: 'Amos',
  Obadiah: 'Obadiah',
  Jonah: 'Jonah',
  Micah: 'Micah',
  Nahum: 'Nahum',
  Habakkuk: 'Habakkuk',
  Zephaniah: 'Zephaniah',
  Haggai: 'Haggai',
  Zechariah: 'Zechariah',
  Malachi: 'Malachi',
  'The Gospel According to Saint Matthew': 'Matthew',
  'The Gospel According to Saint Mark': 'Mark',
  'The Gospel According to Saint Luke': 'Luke',
  'The Gospel According to Saint John': 'John',
  'The Acts of the Apostles': 'Acts',
  'The Epistle of Paul the Apostle to the Romans': 'Romans',
  'The First Epistle of Paul the Apostle to the Corinthians': '1 Corinthians',
  'The Second Epistle of Paul the Apostle to the Corinthians': '2 Corinthians',
  'The Epistle of Paul the Apostle to the Galatians': 'Galatians',
  'The Epistle of Paul the Apostle to the Ephesians': 'Ephesians',
  'The Epistle of Paul the Apostle to the Philippians': 'Philippians',
  'The Epistle of Paul the Apostle to the Colossians': 'Colossians',
  'The First Epistle of Paul the Apostle to the Thessalonians':
    '1 Thessalonians',
  'The Second Epistle of Paul the Apostle to the Thessalonians':
    '2 Thessalonians',
  'The First Epistle of Paul the Apostle to Timothy': '1 Timothy',
  'The Second Epistle of Paul the Apostle to Timothy': '2 Timothy',
  'The Epistle of Paul the Apostle to Titus': 'Titus',
  'The Epistle of Paul the Apostle to Philemon': 'Philemon',
  'The Epistle of Paul the Apostle to the Hebrews': 'Hebrews',
  'The General Epistle of James': 'James',
  'The First Epistle General of Peter': '1 Peter',
  'The Second General Epistle of Peter': '2 Peter',
  'The First Epistle General of John': '1 John',
  'The Second Epistle General of John': '2 John',
  'The Third Epistle General of John': '3 John',
  'The General Epistle of Jude': 'Jude',
  'The Revelation of Saint John the Devine': 'Revelation',
};

const startVerse = /^(\d+):(\d+)/;

const download: Downloader = async ({ d }) => {
  d('Downloading text');

  const { body: rawText } = await got(URL);
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
  const verses: Array<Verse> = [];

  {
    let book = '';
    let chapter = 0;
    let verse = 0;
    let text = '';

    for (const line of lines) {
      if (nameMap[line] && !booksSeen[line]) {
        book = nameMap[line];
        booksSeen[line] = true;
      }
      if (!book) continue;
      if (startVerse.test(line)) {
        // New verse
        const [, sChapter, sVerse] = startVerse.exec(line);
        chapter = parseInt(sChapter, 10);
        verse = parseInt(sVerse, 10);
        text += line.replace(startVerse, '').trim();
      } else if (text && line) {
        text += ` ${line.trim()}`;
      } else if (text) {
        verses.push({
          book,
          chapter,
          verse,
          text,
        });
        chapter = 0;
        verse = 0;
        text = '';
      }
    }
  }

  return verses;
};

base('KJV', download);
