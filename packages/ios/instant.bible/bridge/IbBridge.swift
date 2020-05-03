import Foundation

// The logic in this file brought to you by:
// https://medium.com/visly/rust-on-ios-39f799b3c1dd
//   https://archive.is/g29Q5
// https://github.com/mozilla/application-services/blob/b3c17d3912721ef68fa1e06b2458dcfcce831d36/docs/howtos/passing-protobuf-data-over-ffi.md
//   https://archive.is/f0uQo

extension Data {
    init(buffer: IbRustBuffer) {
        self.init(bytes: buffer.data!, count: Int(buffer.len))
    }
}

class IbBridge {
    static func initialize(data: Data) {
        let count = UInt(data.count)
        
        data.withUnsafeBytes { (bytes: UnsafePointer<UInt8>) in
            bridge_init(bytes, count)
        }
    }
    
    static func search(query: String) -> Instantbible_Service_Response {
        let buf = bridge_search(query)
        let res = Data(buffer: buf)
        let results_proto = try! Instantbible_Service_Response(serializedData: res)
        bridge_search_free(buf)
        return results_proto
    }
}
