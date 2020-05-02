import SwiftUI

struct IbProgressBar: View {
    @Binding var value: Double
    
    var body: some View {
        GeometryReader { geometry in
            HStack {
                Text("\(self.value * 100, specifier: "%.0f")%")
                    .frame(width: 44)
                    .foregroundColor(Color.ibText)
                ZStack(alignment: .leading) {
                    Rectangle()
                        .frame(width: geometry.size.width - 70, height: 8.0)
                        .cornerRadius(4.0)
                        .foregroundColor(Color.ibProgoressBack)
                    Rectangle()
                        .frame(width: (geometry.size.width - 70) * CGFloat(self.value), height: 8.0)
                        .cornerRadius(4.0)
                        .foregroundColor(Color.ibProgressGreen)
                }
            }
            .frame(width: geometry.size.width)
        }.frame(height: 20.0)
    }
}

struct IbProgressBar_Previews: PreviewProvider {
    static var previews: some View {
        IbProgressBar(value: .constant(0.42))
    }
}
