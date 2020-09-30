import SwiftUI

struct IbSettingsSheet: View {
    @Environment(\.presentationMode) var presentationMode
    @Binding var offlineEnabled: Bool
    @Binding var downloadProgress: Double
    @Binding var downloading: Bool
    @Binding var downloadSize: Int64
    @State private var showingLicenses: Bool = false
    
    var body: some View {
        ScrollView {
            VStack(alignment: .leading) {
                Text("Settings").font(.largeTitle)
                
                Toggle(isOn: self.$offlineEnabled) {
                    Text("Enable Offline Mode").font(.headline)
                }
                .disabled(self.downloading)
                
                (
                    Text("Store the search index locally and search without making web requests (i.e., make instant.bible even ")
                        + Text("instanter").italic()
                        + Text(").")
                ).padding(.vertical)
                
                if (self.downloadSize > 0) {
                    Text("Download size: \(ByteCountFormatter.string(fromByteCount: self.downloadSize, countStyle: .file))")
                        .padding(.top)
                }
                
                if (self.offlineEnabled) {
                    IbProgressBar(value: self.$downloadProgress)
                        .padding(.top)
                }
                
                Button("Open Source Software Disclosure") {
                    self.showingLicenses.toggle()
                }
                    .padding(.vertical)
                
                if (self.showingLicenses) {
                    Text("instant.bible uses the following open source software:")
                        .padding(.vertical)
                    Text(" · Alamofire (MIT) https://github.com/Alamofire/Alamofire")
                    Text(" · SwiftUI-Introspect (MIT) https://github.com/siteline/SwiftUI-Introspect")
                    Text(" · sentry-cocoa (MIT) https://github.com/getsentry/sentry-cocoa")
                    Text(" · swift-protobuf (Apache 2.0) https://github.com/apple/swift-protobuf")
                    Text(" · fst (MIT) https://github.com/burntsushi/fst")
                    Text(" · Font Awesome 5 (CC BY 4.0) https://fontawesome.com/license/free")
                }
                
                HStack {
                    Spacer()
                    Button("Close") {
                        self.presentationMode.wrappedValue.dismiss()
                    }
                    .foregroundColor(Color.ibText)
                    .padding()
                    Spacer()
                }
                .padding(.vertical, 20)
                Spacer()
            }
            .padding()
        }
            .background(Color.ibBackground.edgesIgnoringSafeArea(.all))
            .transition(.slide)
    }
}

struct IbSettingsSheet_Previews: PreviewProvider {
    static var previews: some View {
        IbSettingsSheet(offlineEnabled: .constant(true), downloadProgress: .constant(0.42), downloading: .constant(true), downloadSize: .constant(110280328))
    }
}
