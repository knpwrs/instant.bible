package bible.instant.ui.main

import android.content.Context
import android.os.Bundle
import android.util.Log
import androidx.fragment.app.Fragment
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.databinding.DataBindingUtil
import androidx.lifecycle.Observer
import bible.instant.R
import bible.instant.databinding.SettingsFragmentBinding
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import java.io.FileOutputStream
import java.lang.Exception
import java.net.HttpURLConnection
import java.net.URL

const val INDEX_URL = "https://instantbible.nyc3.digitaloceanspaces.com/${MainViewModel.INDEX_FILE}"

class SettingsFragment : Fragment() {

    companion object {
        fun newInstance() = SettingsFragment()
    }

    private lateinit var binding: SettingsFragmentBinding
    private lateinit var viewModel: SettingsViewModel

    override fun onCreateView(
        inflater: LayoutInflater, container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View? {
        binding = DataBindingUtil.inflate(inflater, R.layout.settings_fragment, container, false)

        viewModel = SettingsViewModel(getPrefOffline())
        binding.viewModel = viewModel
        binding.lifecycleOwner = viewLifecycleOwner

        viewModel.offlineEnabled.observe(viewLifecycleOwner, Observer {
            if (it) {
                GlobalScope.launch {
                    downloadIndex()
                    MainViewModel.loadAndInitIndex()
                }
            } else {
                MainViewModel.deleteIndexFile()
                viewModel.resetProgress()
            }

            val sharedPref = activity?.getPreferences(Context.MODE_PRIVATE)
            val editor = sharedPref?.edit()
            editor?.putBoolean(getString(R.string.offline_mode_enabled), it)
            editor?.commit()
        })

        GlobalScope.launch {
            getIndexSize()
        }

        return binding.root
    }

    private fun getPrefOffline(): Boolean {
        val sharedPref = activity?.getPreferences(Context.MODE_PRIVATE)

        return sharedPref?.getBoolean(getString(R.string.offline_mode_enabled), false) ?: false
    }

    private suspend fun downloadIndex() {
        if (MainViewModel.localIndexExists()) {
            viewModel.updateProgress(1, 1)

            return
        }

        viewModel.startDownloading()

        withContext(Dispatchers.IO) {
            val url = URL(INDEX_URL)
            val conn = url.openConnection() as HttpURLConnection
            conn.requestMethod = "GET"
            // Do not set `doOutput` to `true`, it forces the method to POST!

            val file = MainViewModel.getIndexFile()
            val o = FileOutputStream(file)

            try {
                conn.connect()
                val i = conn.inputStream

                val buf = ByteArray(1024)
                val totalBytes = conn.contentLength
                var readBytes = 0

                while (true) {
                    val len = i.read(buf)
                    if (len > 0) {
                        o.write(buf, 0, len)
                        readBytes += len
                        viewModel.updateProgress(readBytes, totalBytes)
                    } else {
                        break
                    }
                }
            } catch (e: Exception) {
                file.delete()
                viewModel.resetProgress()
                Log.i("SF::downloadIndex", "Caught exception: $e")
            } finally {
                conn.disconnect()
                o.close()
                viewModel.finishDownloading()
            }
        }
    }

    private suspend fun getIndexSize() {
        withContext(Dispatchers.IO) {
            val url = URL(INDEX_URL)
            val conn = url.openConnection() as HttpURLConnection
            conn.requestMethod = "HEAD"

            try {
                conn.connect()
                val totalBytes = conn.contentLength
                viewModel.setIndexSize(totalBytes)
            } catch (e: Exception) {
                Log.i("SF::getIndexSize", "Caught exception: $e")
            } finally {
                conn.disconnect()
            }
        }
    }
}
