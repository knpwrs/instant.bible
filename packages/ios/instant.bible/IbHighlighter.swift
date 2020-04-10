import SwiftUI

// Based on: https://stackoverflow.com/a/59427912/355325
// Changes:
//  1. case-insensitive regex with proper subsitution
//  2. word-start splitting (like the web app)
//  3. multiple words

struct IbHighlighter: View {
    let text: String
    let words: [String]
    
    var body: some View {
        let tagged = words.reduce(text, { str, word in
            str.replacingOccurrences(of: "\\b(\(word))", with: "<H>>$1<H>", options: [.caseInsensitive, .regularExpression])
        })
        let split = tagged.components(separatedBy: "<H>")
        return split.reduce(Text("")) { (a, b) -> Text in
            guard !b.hasPrefix(">") else {
                return a + Text(b.dropFirst()).foregroundColor(Color.ibTextHighlight).fontWeight(.bold)
            }
            return a + Text(b).foregroundColor(Color.ibText)
        }
    }
}

struct IbHighlighter_Previews: PreviewProvider {
    static var previews: some View {
        IbHighlighter(text: "For this is the way God loved the world: He gave his one and only Son, so that everyone who believes in him will not perish but have eternal life.", words: ["HIS", "ETERNAL"])
    }
}
