import SwiftUI

class ContentViewModel: ObservableObject {
    let client = IbApiClient()
    
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
                client.search(q: trimmed) { response in
                    self.searchCache[trimmed] = response.results
                }
            }
        }
    }
}

struct IbContentView: View {
    @ObservedObject var model: ContentViewModel = ContentViewModel();
    
    var body: some View {
        ScrollView {
            TextField("Search", text: $model.searchText).padding()
            ForEach(self.model.results, id: \.self) { result in
                IbVerseResultView(result: result)
            }
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        IbContentView()
    }
}
