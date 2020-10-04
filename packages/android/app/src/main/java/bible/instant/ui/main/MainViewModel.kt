package bible.instant.ui.main

import android.util.Log
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import bible.instant.InstantBibleApplication
import bible.instant.network.InstantBibleApi
import instantbible.service.Service
import retrofit2.Call
import retrofit2.Callback
import retrofit2.Response
import java.io.File

class MainViewModel : ViewModel() {
    private val resultsCache = HashMap<String, Service.Response>()
    private var query = ""

    val dirty = MutableLiveData<Boolean>(false)
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
        dirty.value = true
        query = q

        if (resultsCache.containsKey(q) || q == "") {
            count.value = count.value?.inc()
            return
        }

        if (OFFLINE_INITIATED) {
            Log.i("MainViewModel", "SEARCHING OFFLINE $q")
            val resBytes = bridgeSearch(q)
            val res = Service.Response.parseFrom(resBytes)
            resultsCache[q] = res
            count.value = count.value?.inc()
        } else {
            Log.i("MainViewModel", "SEARCHING ONLINE $q")
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

    companion object {
        const val INDEX_FILE = "index.pb"
        private var OFFLINE_INITIATED = false

        init {
            System.loadLibrary("bridge_c")
            loadAndInitIndex()
        }

        fun loadAndInitIndex() {
            // Do not allow double initialization
            if (OFFLINE_INITIATED) {
                return
            }

            val indexData = loadIndex()
            Log.i("MainViewModel", "Index data length ${indexData?.size}")
            if (indexData != null) {
                bridgeInit(indexData)
                OFFLINE_INITIATED = true
            }
        }

        private fun loadIndex(): ByteArray? {
            if (localIndexExists()) {
                return getIndexFile().readBytes()
            }

            return null
        }

        fun deleteIndexFile() {
            getIndexFile().delete()
        }

        fun localIndexExists(): Boolean {
            return getIndexFile().exists()
        }

        fun getIndexFile(): File {
            return File(InstantBibleApplication.context?.filesDir, INDEX_FILE)
        }

        @JvmStatic
        external fun bridgeInit(bytes: ByteArray)

        @JvmStatic
        external fun bridgeSearch(q: String): ByteArray
    }
}
