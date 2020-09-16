package bible.instant

import instantbible.data.Data

fun getTranslationLabel(translation: Int): String {
    return when (translation) {
        Data.Translation.BSB_VALUE -> "BSB"
        Data.Translation.KJV_VALUE -> "KJV"
        Data.Translation.NET_VALUE -> "NET"
        else -> "UNRC"
    }
}

fun getBookName(book: Data.Book): String {
    return when (book) {
        Data.Book.GENESIS -> "Genesis"
        Data.Book.EXODUS -> "Exodus"
        Data.Book.LEVITICUS -> "Leviticus"
        Data.Book.NUMBERS -> "Numbers"
        Data.Book.DEUTERONOMY -> "Deuteronomy"
        Data.Book.JOSHUA -> "Joshua"
        Data.Book.JUDGES -> "Judges"
        Data.Book.RUTH -> "Ruth"
        Data.Book.FIRST_SAMUEL -> "1 Samuel"
        Data.Book.SECOND_SAMUEL -> "2 Samuel"
        Data.Book.FIRST_KINGS -> "1 Kings"
        Data.Book.SECOND_KINGS -> "2 Kings"
        Data.Book.FIRST_CHRONICLES -> "1 Chronicles"
        Data.Book.SECOND_CHRONICLES -> "2 Chronicles"
        Data.Book.EZRA -> "Ezra"
        Data.Book.NEHEMIAH -> "Nehemiah"
        Data.Book.ESTHER -> "Esther"
        Data.Book.JOB -> "Job"
        Data.Book.PSALMS -> "Psalms"
        Data.Book.PROVERBS -> "Proverbs"
        Data.Book.ECCLESIASTES -> "Ecclesiastes"
        Data.Book.SONG_OF_SOLOMON -> "Song of Solomon"
        Data.Book.ISAIAH -> "Isaiah"
        Data.Book.JEREMIAH -> "Jeremiah"
        Data.Book.LAMENTATIONS -> "Lamentations"
        Data.Book.EZEKIEL -> "Ezekiel"
        Data.Book.DANIEL -> "Daniel"
        Data.Book.HOSEA -> "Hosea"
        Data.Book.JOEL -> "Joel"
        Data.Book.AMOS -> "Amos"
        Data.Book.OBADIAH -> "Obadiah"
        Data.Book.JONAH -> "Jonah"
        Data.Book.MICAH -> "Micha"
        Data.Book.NAHUM -> "Nahum"
        Data.Book.HABAKKUK -> "Habakkuk"
        Data.Book.ZEPHANIAH -> "Zephaniah"
        Data.Book.HAGGAI -> "Haggai"
        Data.Book.ZECHARIAH -> "Zechariah"
        Data.Book.MALACHI -> "Malachi"
        Data.Book.MATTHEW -> "Matthew"
        Data.Book.MARK -> "Mark"
        Data.Book.LUKE -> "Luke"
        Data.Book.JOHN -> "John"
        Data.Book.ACTS -> "Acts"
        Data.Book.ROMANS -> "Romans"
        Data.Book.FIRST_CORINTHIANS -> "1 Corinthians"
        Data.Book.SECOND_CORINTHIANS -> "2 Corinthians"
        Data.Book.GALATIANS -> "Galatians"
        Data.Book.EPHESIANS -> "Ephesians"
        Data.Book.PHILIPPIANS -> "Philippians"
        Data.Book.COLOSSIANS -> "Colossians"
        Data.Book.FIRST_THESSALONIANS -> "1 Thessalonians"
        Data.Book.SECOND_THESSALONIANS -> "2 Thessalonians"
        Data.Book.FIRST_TIMOTHY -> "1 Timothy"
        Data.Book.SECOND_TIMOTHY -> "2 Timothy"
        Data.Book.TITUS -> "Titus"
        Data.Book.PHILEMON -> "Philemon"
        Data.Book.HEBREWS -> "Hebrews"
        Data.Book.JAMES -> "James"
        Data.Book.FIRST_PETER -> "1 Peter"
        Data.Book.SECOND_PETER -> "2 Peter"
        Data.Book.FIRST_JOHN -> "1 John"
        Data.Book.SECOND_JOHN -> "2 John"
        Data.Book.THIRD_JOHN -> "3 John"
        Data.Book.JUDE -> "Jude"
        Data.Book.REVELATION -> "Revelation"
        Data.Book.UNRECOGNIZED -> "Unrecognized"
    }
}
