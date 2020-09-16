package bible.instant

import android.app.Application
import android.content.Context

class InstantBibleApplication: Application() {

    companion object {
        var context: Context? = null
    }

    override fun onCreate() {
        super.onCreate()
        context = this
    }
}
