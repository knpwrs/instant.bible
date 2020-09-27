package bible.instant.ui.main

import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import kotlin.math.round

class SettingsViewModel(offline: Boolean = false) : ViewModel() {
    val offlineEnabled = MutableLiveData(offline)
    val downloading = MutableLiveData(false)
    val progress = MutableLiveData(0)
    val indexSizeBytes = MutableLiveData(0)
    val showingOpenSource = MutableLiveData(false)

    fun toggleOffline() {
        val nowEnabled = !(offlineEnabled.value ?: false)
        offlineEnabled.value = nowEnabled
    }

    fun toggleOpenSource() {
        val nowEnabled = !(showingOpenSource.value ?: false)
        showingOpenSource.value = nowEnabled
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
    }

    fun resetProgress() {
        progress.postValue(0)
    }

    fun setIndexSize(bytes: Int) {
        indexSizeBytes.postValue(bytes)
    }
}
