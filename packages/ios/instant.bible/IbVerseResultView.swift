import SwiftUI

class IbVerseResultViewModel: ObservableObject {
    let result: Instantbible_Service_Response.VerseResult;
    @Published var selectedTranslation: Instantbible_Data_Translation;
    
    init(result: Instantbible_Service_Response.VerseResult) {
        self.result = result;
        self.selectedTranslation = result.topTranslation;
    }
}

struct IbVerseResultView: View {
    @ObservedObject var model: IbVerseResultViewModel
    
    init(result: Instantbible_Service_Response.VerseResult) {
        self.model = IbVerseResultViewModel(result: result)
    }
    
    var body: some View {
        VStack(alignment: .leading) {
            Text("\(IbBookNameMap[self.model.result.key.book] ?? "Unknown Book") \(self.model.result.key.chapter):\(self.model.result.key.verse)").bold()
            IbHighlighter(text: self.model.result.text[self.model.selectedTranslation.rawValue], words: self.model.result.highlights)
                .padding(.vertical)
            HStack {
                IbTranslationButton(translation: .kjv, selectedTranslation: self.model.selectedTranslation) {
                    self.model.selectedTranslation = .kjv
                }
                IbTranslationButton(translation: .net, selectedTranslation: self.model.selectedTranslation) {
                    self.model.selectedTranslation = .net
                }
            }
        }
        .padding()
        .frame(maxWidth: .infinity, alignment: .topLeading)
        .background(Color.ibCard)
        .cornerRadius(10)
    }
}

struct IbVerseResultView_Previews: PreviewProvider {
    static var previews: some View {
        ZStack {
            Rectangle().fill(Color.ibBackground)
            IbVerseResultView(result: .with {
                $0.key = .with {
                    $0.book = .john
                    $0.chapter = 3
                    $0.verse = 16
                }
                $0.topTranslation = .kjv
                $0.text = [
                    "For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life.",
                    "For this is the way God loved the world: He gave his one and only Son, so that everyone who believes in him will not perish but have eternal life.",
                ]
                $0.highlights = ["HIS", "LIFE"]
            })
        }
    }
}
