package bible.instant.network

import instantbible.service.Service
import retrofit2.Call
import retrofit2.Retrofit
import retrofit2.converter.protobuf.ProtoConverterFactory
import retrofit2.http.GET
import retrofit2.http.Headers
import retrofit2.http.Query

private const val BASE_URL = "https://api.instant.bible"

private val retrofit =
    Retrofit.Builder().baseUrl(BASE_URL).addConverterFactory(ProtoConverterFactory.create()).build()

interface InstantBibleApiService {
    @Headers("Accept: application/protobuf")
    @GET("/")
    fun search(@Query("q") query: String): Call<Service.Response>
}

object InstantBibleApi {
    val retrofitService: InstantBibleApiService by lazy {
        retrofit.create(InstantBibleApiService::class.java)
    }
}
