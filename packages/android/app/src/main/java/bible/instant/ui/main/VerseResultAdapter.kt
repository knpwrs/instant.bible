package bible.instant.ui.main

import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.Button
import android.widget.LinearLayout
import android.widget.TextView
import androidx.recyclerview.widget.RecyclerView
import bible.instant.R
import bible.instant.getBookName
import bible.instant.getTranslationLabel
import instantbible.data.Data
import instantbible.service.Service

class VerseResultViewHolder(itemView: View) : RecyclerView.ViewHolder(itemView) {
    val verseTitle: TextView = itemView.findViewById(R.id.verse_title)
    val verseText: TextView = itemView.findViewById(R.id.verse_text)
    val translationsHolder: LinearLayout = itemView.findViewById(R.id.translations)
}

class VerseResultAdapter : RecyclerView.Adapter<VerseResultViewHolder>() {
    var data = listOf<Service.Response.VerseResult>()
        set(value) {
            field = value
            notifyDataSetChanged()
        }

    override fun getItemCount() = data.size

    override fun onBindViewHolder(holder: VerseResultViewHolder, position: Int) {
        val item = data[position]

        holder.verseTitle.text = "${getBookName(item.key.book)} ${item.key.chapter}:${item.key.verse}"
        holder.verseText.text = item.getText(item.topTranslationValue)

        for (t in 0 until holder.translationsHolder.childCount) {
            val btn = holder.translationsHolder.getChildAt(t)
            btn.setOnClickListener {
                holder.verseText.text = item.getText(btn.getTag(R.string.translation_tag) as Int)
            }
        }
    }

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): VerseResultViewHolder {
        val layoutInflater = LayoutInflater.from(parent.context)
        val view = layoutInflater.inflate(R.layout.verse_result_view, parent, false)
        val translationsHolder: LinearLayout = view.findViewById(R.id.translations)

        for (t in 0 until Data.Translation.TOTAL_VALUE) {
            val btn = Button(view.context)
            btn.text = getTranslationLabel(t)
            btn.setTag(R.string.translation_tag, t)
            translationsHolder.addView(btn)
        }

        return VerseResultViewHolder(view)
    }
}

