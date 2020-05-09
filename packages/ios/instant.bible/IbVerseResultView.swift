import SwiftUI

class IbVerseResultViewModel: ObservableObject {
    let result: Instantbible_Service_Response.VerseResult;
    @Published var selectedTranslation: Instantbible_Data_Translation;
    
    init(result: Instantbible_Service_Response.VerseResult) {
        self.result = result;
        self.selectedTranslation = result.topTranslation;
    }
    
    func getTitle() -> String {
        "\(IbBookNameMap[self.result.key.book] ?? "Unknown Book") \(self.result.key.chapter):\(self.result.key.verse)"
    }
    
    func getBody() -> String {
        self.result.text[self.selectedTranslation.rawValue]
    }
    
    func getCopyText() -> String {
        "\(self.getTitle()) \(IbTranslationNameMap[self.selectedTranslation] ?? "INV")\n\(self.getBody())"
    }
}

struct IbVerseResultActivityView: UIViewControllerRepresentable {
    let activityItems: [Any]
    let applicationActivities: [UIActivity]?

    func makeUIViewController(context: UIViewControllerRepresentableContext<IbVerseResultActivityView>) -> UIActivityViewController {
        return UIActivityViewController(activityItems: activityItems,
                                        applicationActivities: applicationActivities)
    }

    func updateUIViewController(_ uiViewController: UIActivityViewController,
                                context: UIViewControllerRepresentableContext<IbVerseResultActivityView>) {
    }
}

var translationKeys = Array(IbTranslationNameMap.keys).sorted { IbTranslationNameMap[$0]! < IbTranslationNameMap[$1]! }

struct IbVerseResultView: View {
    @ObservedObject var model: IbVerseResultViewModel
    @State var showingActions: Bool = false
    
    init(result: Instantbible_Service_Response.VerseResult) {
        self.model = IbVerseResultViewModel(result: result)
    }
    
    var body: some View {
        VStack(alignment: .leading) {
            Text(self.model.getTitle()).bold()
            IbHighlighter(text: self.model.getBody(), words: self.model.result.highlights)
                .padding(.vertical)
            HStack {
                ForEach(translationKeys, id: \.self) { key in
                    IbTranslationButton(translation: key, selectedTranslation: self.model.selectedTranslation) {
                        self.model.selectedTranslation = key
                    }
                }
            }
        }
        .padding()
        .frame(maxWidth: .infinity, alignment: .topLeading)
        .background(Color.ibCard)
        .cornerRadius(10)
        .onTapGesture {
            // This empty handler is required to make the ScrollView still scrollable
            // https://stackoverflow.com/a/59499892/355325
        }
        .onLongPressGesture {
            self.showingActions = true
        }
        .sheet(isPresented: $showingActions,
               content: {
                IbVerseResultActivityView(activityItems: [self.model.getCopyText()] as [Any], applicationActivities: nil) })
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
                    "For God so loved the world that He gave His one and only Son, that everyone who believes in Him shall not perish but have eternal life.",
                    "For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life.",
                    "For this is the way God loved the world: He gave his one and only Son, so that everyone who believes in him will not perish but have eternal life.",
                ]
                $0.highlights = ["HIS", "LIFE"]
            })
        }
    }
}
