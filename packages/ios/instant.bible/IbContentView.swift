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
    
    @Published var downloading: Bool = false
    @Published var downloadProgress: Double = 0.0
    
    var offlineInitted = false
    @Published var offlineEnabled = false {
        willSet(newOfflineEnabled) {
            if (newOfflineEnabled) {
                self.downloading = true
                IbNet.loadIndex(onProgress: { progress in
                    self.downloadProgress = progress
                }, onSuccess: { data in
                    self.downloading = false
                    if !self.offlineInitted {
                        IbBridge.initialize(data: data)
                        self.offlineInitted = true
                    }
                    UserDefaults.standard.set(newOfflineEnabled, forKey: "offlineEnabled")
                })
            } else {
                UserDefaults.standard.set(newOfflineEnabled, forKey: "offlineEnabled")
            }
        }
    }
    
    @Published var searchText = "" {
        willSet(newSearchText) {
            let trimmed = newSearchText.trimmingCharacters(in: .whitespacesAndNewlines)
            self.resultsKey = trimmed
            
            if searchCache[trimmed] == nil {
                if offlineInitted {
                    let response = IbBridge.search(query: trimmed)
                    self.searchCache[trimmed] = response.results
                } else {
                    IbNet.apiSearch(q: trimmed) { response in
                        self.searchCache[trimmed] = response.results
                    }
                }
            }
        }
    }
    
    init() {
        // Initialize here so willSet logic fires
        self.offlineEnabled = UserDefaults.standard.bool(forKey: "offlineEnabled")
    }
}

struct IbContentView: View {
    @ObservedObject var model: ContentViewModel = ContentViewModel();
    @State var showingSettings = false
    
    var body: some View {
        VStack {
            HStack {
                TextField("Search", text: $model.searchText)
                    .foregroundColor(Color.ibText)
                    .padding(.vertical)
                    .padding(.leading, 20)
                Button(action: {
                    self.showingSettings = true
                }) {
                    Image("fa-bars-solid").foregroundColor(Color.ibText)
                }.frame(width: 44, height: 44)
            }
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
        .sheet(isPresented: $showingSettings, content: {
            IbSettingsSheet(offlineEnabled: self.$model.offlineEnabled, downloadProgress: self.$model.downloadProgress, downloading: self.$model.downloading)
        })
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
