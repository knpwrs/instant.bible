import SwiftUI

class ContentViewModel: ObservableObject {
    @Published private var searchCache: [String: [Instantbible_Service_Response.VerseResult]] = [:]
    @Published private var resultsKey = ""
    
    var results: [Instantbible_Service_Response.VerseResult] {
        for i in (0..<resultsKey.count).reversed() {
            let start = String.Index(utf16Offset: 0, in: resultsKey)
            let end = String.Index(utf16Offset: i, in: resultsKey)
            let key = String(resultsKey[start...end])
            if let cachedResults = searchCache[key] {
                return cachedResults
            }
        }
        return []
    }
    
    @Published var searchText = "" {
        willSet(newSearchText) {
            let trimmed = newSearchText.trimmingCharacters(in: .whitespacesAndNewlines)
            self.resultsKey = trimmed
            
            if searchCache[trimmed] == nil {
                IbNet.apiSearch(q: trimmed) { response in
                    self.searchCache[trimmed] = response.results
                }
            }
        }
    }
}

struct IbContentView: View {
    @ObservedObject var model: ContentViewModel = ContentViewModel();
    
    var body: some View {
        VStack {
            TextField("Search", text: $model.searchText)
                .padding()
            ScrollView(.vertical) {
                VStack(spacing: 18) {
                    ForEach(self.model.results, id: \.self) { result in
                        IbVerseResultView(result: result)
                            .padding(.horizontal)
                    }
                }
                .frame(maxWidth: .infinity)
            }
            .resignKeyboardOnDragGesture()
        }
        .background(Color.ibBackground.edgesIgnoringSafeArea(.all))
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        IbContentView()
    }
}

// From https://github.com/UPetersen/SwiftUI-SearchBar
extension UIApplication {
    func endEditing(_ force: Bool) {
        self.windows
            .filter{$0.isKeyWindow}
            .first?
            .endEditing(force)
    }
}

struct ResignKeyboardOnDragGesture: ViewModifier {
    var gesture = DragGesture().onChanged{_ in
        UIApplication.shared.endEditing(true)
    }
    func body(content: Content) -> some View {
        content.gesture(gesture)
    }
}

extension View {
    func resignKeyboardOnDragGesture() -> some View {
        modifier(ResignKeyboardOnDragGesture())
    }
}
