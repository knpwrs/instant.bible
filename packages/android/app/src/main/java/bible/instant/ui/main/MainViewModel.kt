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
    val results = MutableLiveData<Service.Response>()

    fun doSearch(query: String) {
        InstantBibleApi.retrofitService.search(query).enqueue(object: Callback<Service.Response> {
            override fun onFailure(call: Call<Service.Response>, t: Throwable) {
                Log.e("Error", "Error handling response: ${t.message}")
            }

            override fun onResponse(
                call: Call<Service.Response>,
                response: Response<Service.Response>
            ) {
                response?.body()?.let {
                    results.value = it
                }
            }

        })
    }
}
