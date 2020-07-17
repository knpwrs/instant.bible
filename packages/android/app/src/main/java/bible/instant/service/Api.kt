package bible.instant.service

import instantbible.service.Service
import retrofit2.Call
import retrofit2.Retrofit
import retrofit2.converter.protobuf.ProtoConverterFactory
import retrofit2.http.GET
import retrofit2.http.Headers
import retrofit2.http.Query

private const val BASE_URL="https://api.instant.bible"

private val retrofit = Retrofit.Builder()
  .addConverterFactory(ProtoConverterFactory.create())
  .baseUrl(BASE_URL)
  .build()

interface IbApiService {
  @GET("/")
  @Headers("Accept: application/protobuf")
  fun search(@Query("q") query: String): Call<Service.Response>
}

object IbApi {
  val ibApiService: IbApiService by lazy {
    retrofit.create(IbApiService::class.java)
  }
}
