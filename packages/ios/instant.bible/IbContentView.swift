import SwiftUI
import Introspect

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
    @Published var downloadSize: Int64 = 0
    
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
                self.downloadProgress = 0
                IbNet.deleteIndex()
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
        IbNet.getIndexSize(onSuccess: { size in
            self.downloadSize = size
        })
    }
}

struct IbContentView: View {
    @ObservedObject var model: ContentViewModel = ContentViewModel();
    @State var showingSettings = false
    
    var body: some View {
        ZStack(alignment: .bottomTrailing) {
            VStack {
                TextField("Search", text: $model.searchText)
                    .introspectTextField { textfield in
                        textfield.becomeFirstResponder()
                    }
                    .foregroundColor(Color.ibText)
                    .padding(.vertical)
                    .padding(.leading, 20)
                    .padding(.trailing, 20)
                ScrollView(.vertical) {
                    VStack(spacing: 18) {
                        ForEach(self.model.results, id: \.self) { result in
                            IbVerseResultView(result: result)
                                .padding(.horizontal)
                        }
                        if (!self.model.results.isEmpty) {
                            Text("The Holy Bible, Berean Study Bible, BSB Copyright ©2016, 2018, 2020 by Bible Hub Used by Permission. All Rights Reserved Worldwide.")
                                .foregroundColor(.ibTextSecondary)
                                .multilineTextAlignment(.center)
                                .padding(.horizontal)
                            Text("The Holy Bible, King James Version, KJV is in the public domain and not subject to copyright.")
                                .foregroundColor(.ibTextSecondary)
                                .multilineTextAlignment(.center)
                                .padding(.horizontal)
                            Text("The NET Bible® https://netbible.com copyright ©1996, 2019 used with permission from Biblical Studies Press, L.L.C. All rights reserved.")
                                .foregroundColor(.ibTextSecondary)
                                .multilineTextAlignment(.center)
                                .padding(.horizontal)
                        }
                    }
                    // Leave space for FAB to clear last card
                    .padding(.bottom, 100)
                    .frame(maxWidth: .infinity)
                }
                .resignKeyboardOnDragGesture()
            }
            .background(Color.ibBackground.edgesIgnoringSafeArea(.all))
            // Settings FAB
            Button(action: {
                self.showingSettings = true
            }) {
                Image("fa-cog-solid")
                    .background(
                        ZStack {
                            Circle()
                                .fill(Color.ibShadow)
                                .frame(width: 62, height: 62, alignment: /*@START_MENU_TOKEN@*/.center/*@END_MENU_TOKEN@*/)
                            Circle()
                                .fill(Color.ibCard)
                                .frame(width: 60, height: 60, alignment: .center)
                        }
                    )
                    .padding()
            }
                
            .frame(width: 80, height: 80)
            .sheet(isPresented: $showingSettings, content: {
                IbSettingsSheet(offlineEnabled: self.$model.offlineEnabled, downloadProgress: self.$model.downloadProgress, downloading: self.$model.downloading, downloadSize: self.$model.downloadSize)
            })
        }
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
