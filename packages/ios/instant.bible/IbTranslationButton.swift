import SwiftUI

enum IbTranslationButtonError: Error {
    case invalidTranslation
}

struct IbTranslationButton: View {
    var translation: Instantbible_Data_Translation
    var selectedTranslation: Instantbible_Data_Translation
    var action: () -> Void
    
    var body: some View {
        Button(IbTranslationNameMap[self.translation] ?? "INV", action: self.action)
            .font(Font.body.bold())
            .foregroundColor(Color.ibText)
            .opacity(self.translation == self.selectedTranslation ? 1.0 : 0.65)
    }
}

struct IbTranslationButton_Previews: PreviewProvider {
    static var previews: some View {
        VStack {
            IbTranslationButton(translation: .kjv, selectedTranslation: .kjv) {
                print("First Button")
            }
            IbTranslationButton(translation: .net, selectedTranslation: .net) {
                print("Second Button")
            }
        }
    }
}
