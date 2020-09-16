package bible.instant.ui.main

import android.util.Log
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import kotlin.math.round

class SettingsViewModel(offline: Boolean = false) : ViewModel() {
    val offlineEnabled = MutableLiveData(offline)
    val downloading = MutableLiveData(false)
    val progress = MutableLiveData(0)

    fun toggleOffline() {
        val nowEnabled = !(offlineEnabled.value ?: false)
        offlineEnabled.value = nowEnabled
    }

    fun startDownloading() {
        downloading.postValue(true)
    }

    fun finishDownloading() {
        downloading.postValue(false)
    }

    fun updateProgress(readBytes: Int, totalBytes: Int) {
        val ratio = readBytes.toDouble() / totalBytes.toDouble()
        val value = ratio * 100
        progress.postValue(round(value).toInt())
        Log.i("SettingsViewModel", "Progress: ${progress.value}")
    }
}
