package bible.instant.ui.main

import android.util.Log
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import bible.instant.network.InstantBibleApi
import instantbible.service.Service
import retrofit2.Call
import retrofit2.Callback
import retrofit2.Response

class MainViewModel : ViewModel() {
    private val resultsCache = HashMap<String, Service.Response>()
    private var query = ""
    val count = MutableLiveData<Int>(0)

    fun getResults(): Service.Response? {
        if (query == "") {
            return null
        }

        for (i in query.indices) {
            val key = query.substring(0, query.length - i)
            if (resultsCache.containsKey(key)) {
                return resultsCache[key]
            }
        }

        return null
    }

    fun doSearch(q: String) {
        query = q

        if (resultsCache.containsKey(q) || q == "") {
            count.value = count.value?.inc()
            return
        }

        InstantBibleApi.retrofitService.search(q).enqueue(object : Callback<Service.Response> {
            override fun onFailure(call: Call<Service.Response>, t: Throwable) {
                Log.e("Error", "Error handling response: ${t.message}")
            }

            override fun onResponse(
                call: Call<Service.Response>,
                response: Response<Service.Response>
            ) {
                response?.body()?.let {
                    resultsCache[q] = it
                    count.value = count.value?.inc()
                }
            }

        })
    }
}
