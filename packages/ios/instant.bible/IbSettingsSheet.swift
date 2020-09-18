import SwiftUI


struct IbSettingsSheet: View {
    @Environment(\.presentationMode) var presentationMode
    @Binding var offlineEnabled: Bool
    @Binding var downloadProgress: Double
    @Binding var downloading: Bool
    @Binding var downloadSize: Int64
    
    var body: some View {
        VStack(alignment: .leading) {
            Text("Settings").font(.largeTitle)
            Toggle(isOn: self.$offlineEnabled) {
                Text("Enable Offline Mode").font(.headline)
            }
            .disabled(self.downloading)
            Text("Store the search index locally and search without making web requests (i.e., make instant.bible even ") + Text("instanter").italic() + Text(").")
            if (self.downloadSize > 0) {
                Text("Download size: \(ByteCountFormatter.string(fromByteCount: self.downloadSize, countStyle: .file)))")
                    .padding(.top)
            }
            if (self.offlineEnabled) {
                IbProgressBar(value: self.$downloadProgress)
                    .padding(.top)
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
        .background(Color.ibBackground.edgesIgnoringSafeArea(.all))
        .transition(.slide)
    }
}

struct IbSettingsSheet_Previews: PreviewProvider {
    static var previews: some View {
        IbSettingsSheet(offlineEnabled: .constant(true), downloadProgress: .constant(0.42), downloading: .constant(true), downloadSize: .constant(110280328))
    }
}
