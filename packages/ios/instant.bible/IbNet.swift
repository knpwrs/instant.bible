import Foundation
import SwiftProtobuf
import Alamofire

class IbNet {
    static let apiUrl = URL(string: Bundle.main.object(forInfoDictionaryKey: "IB_API_BASE") as! String)!
    static let apiHeaders: HTTPHeaders = [.accept("application/protobuf")]
    static let indexUrl = URL(string: Bundle.main.object(forInfoDictionaryKey: "IB_INDEX_URL") as! String)!
    static let localIndexFile = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)[0].appendingPathComponent("index.pb")
    
    static func apiSearch(q: String, onSuccess: @escaping ((Instantbible_Service_Response) -> Void)) {
        var comps = URLComponents(url: self.apiUrl, resolvingAgainstBaseURL: true)!
        comps.queryItems = [URLQueryItem(name: "q", value: q)]
        guard let url = comps.url?.absoluteURL else {
            print("Invalid URL")
            return
        }
        AF.request(url, headers: self.apiHeaders).responseData{ response in
            if let data = response.value {
                onSuccess(try! Instantbible_Service_Response(serializedData: data))
            }
        }
    }
    
    static func loadIndex(onProgress: @escaping ((Double) -> Void), onSuccess: @escaping ((Data) -> Void)) {
        if (FileManager.default.fileExists(atPath: localIndexFile.path)) {
            onProgress(1.0)
            let data = FileManager.default.contents(atPath: localIndexFile.path)!
            onSuccess(data)
        } else {
            downloadIndex(onProgress: onProgress, onSuccess: onSuccess)
        }
    }
    
    static func deleteIndex() {
        if (FileManager.default.fileExists(atPath: localIndexFile.path)) {
            do {
                try FileManager.default.removeItem(at: localIndexFile)
            } catch let error as NSError {
                print("Error: \(error.domain)")
            }
        }
    }
    
    static func downloadIndex(onProgress: @escaping ((Double) -> Void), onSuccess: @escaping ((Data) -> Void)) {
        let destination: DownloadRequest.Destination = { _, _ in
            return (localIndexFile, [.removePreviousFile, .createIntermediateDirectories])
        }
        
        AF.download(self.indexUrl, to: destination)
        .downloadProgress { progress in
            onProgress(progress.fractionCompleted)
        }
        .responseData { response in
            if let data = response.value {
                onSuccess(data)
            }
        }
    }
    
    static func getIndexSize(onSuccess: @escaping ((Int64) -> Void)) {
        AF.request(indexUrl, method: .head).response { res in
            if let lenStr = res.response?.headers.value(for: "Content-Length") {
                if let len = Int64(lenStr) {
                    onSuccess(len)
                }
            }
        }
    }
}
