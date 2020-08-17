package bible.instant.ui.main

import android.util.Log
import android.view.LayoutInflater
import android.view.ViewGroup
import android.widget.TextView
import androidx.recyclerview.widget.RecyclerView
import bible.instant.R
import bible.instant.VerseResultViewHolder
import instantbible.service.Service

class VerseResultAdapter : RecyclerView.Adapter<VerseResultViewHolder>() {
    var data = listOf<Service.Response.VerseResult>()
        set(value) {
            field = value
            notifyDataSetChanged()
        }

    override fun getItemCount() = data.size

    override fun onBindViewHolder(holder: VerseResultViewHolder, position: Int) {
        val item = data[position]
        holder.textView.text = item.getText(item.topTranslationValue)
    }

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): VerseResultViewHolder {
        val layoutInflater = LayoutInflater.from(parent.context)
        val view = layoutInflater.inflate(R.layout.verse_result_view, parent, false) as TextView
        return VerseResultViewHolder(view)
    }
}
