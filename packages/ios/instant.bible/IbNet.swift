import Foundation
import SwiftProtobuf
import Alamofire

class IbNet {
    static let apiUrl = URL(string: Bundle.main.object(forInfoDictionaryKey: "IB_API_BASE") as! String)!
    static let apiHeaders: HTTPHeaders = [.accept("application/protobuf")]

    static func apiSearch(q: String, success: @escaping ((Instantbible_Service_Response) -> Void)) {
        var comps = URLComponents(url: self.apiUrl, resolvingAgainstBaseURL: true)!
        comps.queryItems = [URLQueryItem(name: "q", value: q)]
        guard let url = comps.url?.absoluteURL else {
            print("Invalid URL")
            return
        }
        AF.request(url, headers: self.apiHeaders).responseData{ response in
            if let data = response.value {
                success(try! Instantbible_Service_Response(serializedData: data))
            }
        }
    }
}
