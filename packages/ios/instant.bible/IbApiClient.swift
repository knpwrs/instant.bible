import Foundation
import SwiftProtobuf

class IbApiClient {
    let apiUrl: URL
    let contentType = "application/protobuf"
    
    init(baseUrl: URL = URL(string: Bundle.main.object(forInfoDictionaryKey: "IB_API_BASE") as! String)!) {
        self.apiUrl = baseUrl;
    }
    
    func search(q: String, success: @escaping ((Instantbible_Service_Response) -> Void)) {
        var comps = URLComponents(url: apiUrl, resolvingAgainstBaseURL: true)!
        comps.queryItems = [URLQueryItem(name: "q", value: q)]
        guard let url = comps.url?.absoluteURL else {
            print("Invalid URL")
            return
        }
        get(url: url, success: success)
    }
    
    func get<Response: SwiftProtobuf.Message>(url: URL, success: @escaping ((Response) -> Void)) {
        var request = URLRequest(url: url)
        request.httpMethod = "GET"
        request.setValue("application/protobuf", forHTTPHeaderField: "Accept")
        let task = URLSession.shared.dataTask(with: request) { data, response, error in
            guard let data = data, let response = response as? HTTPURLResponse else {
                print("Error during GET request")
                return
            }
            guard 200..<300 ~= response.statusCode else {
                print("Error from server: \(response.statusCode)")
                return
            }
            let res = try! Response(serializedData: data)
            DispatchQueue.main.async {
                success(res)
            }
        }
        task.resume()
    }
}
