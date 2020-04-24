import Foundation
import SwiftUI

let IbTranslationNameMap: [Instantbible_Data_Translation: String] = [
    .kjv: "KJV",
    .net: "NET",
]

let IbBookNameMap: [Instantbible_Data_Book: String] = [
    .genesis: "Genesis",
    .exodus: "Exodus",
    .leviticus: "Leviticus",
    .numbers: "Numbers",
    .deuteronomy: "Deuteronomy",
    .joshua: "Joshua",
    .judges: "Judges",
    .ruth: "Ruth",
    .firstSamuel: "1 Samuel",
    .secondSamuel: "2 Samuel",
    .firstKings: "1 Kings",
    .secondKings: "2 Kings",
    .firstChronicles: "1 Chronicles",
    .secondChronicles: "2 Chronicles",
    .ezra: "Ezra",
    .nehemiah: "Nehemiah",
    .esther: "Esther",
    .job: "Job",
    .psalms: "Psalms",
    .proverbs: "Proverbs",
    .ecclesiastes: "Ecclesiastes",
    .songOfSolomon: "Song of Solomon",
    .isaiah: "Isaiah",
    .jeremiah: "Jeremiah",
    .lamentations: "Lamentations",
    .ezekiel: "Ezekiel",
    .daniel: "Daniel",
    .hosea: "Hosea",
    .joel: "Joel",
    .amos: "Amos",
    .obadiah: "Obadiah",
    .jonah: "Jonah",
    .micah: "Micha",
    .nahum: "Nahum",
    .habakkuk: "Habakkuk",
    .zephaniah: "Zephaniah",
    .haggai: "Haggai",
    .zechariah: "Zechariah",
    .malachi: "Malachi",
    .matthew: "Matthew",
    .mark: "Mark",
    .luke: "Luke",
    .john: "John",
    .acts: "Acts",
    .romans: "Romans",
    .firstCorinthians: "1 Corinthians",
    .secondCorinthians: "2 Corinthians",
    .galatians: "Galatians",
    .ephesians: "Ephesians",
    .philippians: "Philippians",
    .colossians: "Colossians",
    .firstThessalonians: "1 Thessalonians",
    .secondThessalonians: "2 Thessalonians",
    .firstTimothy: "1 Timothy",
    .secondTimothy: "2 Timothy",
    .titus: "Titus",
    .philemon: "Philemon",
    .hebrews: "Hebrews",
    .james: "James",
    .firstPeter: "1 Peter",
    .secondPeter: "2 Peter",
    .firstJohn: "1 John",
    .secondJohn: "2 John",
    .thirdJohn: "3 John",
    .jude: "Jude",
    .revelation: "Revelation",
]

extension Color {
    static let ibBackground = Color("ibBackground")
    static let ibCard = Color("ibCard")
    static let ibText = Color("ibText")
    static let ibTextHighlight = Color("ibTextHighlight")
    static let ibTextSecondary = Color("ibTextSecondary")
}
