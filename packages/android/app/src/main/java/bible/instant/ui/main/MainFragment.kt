package bible.instant.ui.main

import android.os.Bundle
import androidx.fragment.app.Fragment
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.core.widget.doAfterTextChanged
import androidx.databinding.DataBindingUtil
import androidx.lifecycle.Observer
import bible.instant.R
import bible.instant.databinding.MainFragmentBinding

class MainFragment : Fragment() {
    companion object {
        fun newInstance() = MainFragment()
    }

    private lateinit var binding: MainFragmentBinding
    private lateinit var viewModel: MainViewModel

    override fun onCreateView(
        inflater: LayoutInflater, container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View {
        // Fragment Bootstrapping
        binding = DataBindingUtil.inflate(
            inflater, R.layout.main_fragment, container, false
        )

        // Viewmodel shenanigans
        viewModel = MainViewModel();
        binding.viewModel = viewModel
        binding.lifecycleOwner = viewLifecycleOwner

        // Perform searches
        binding.searchInput.doAfterTextChanged { text ->
            viewModel.doSearch(text.toString())
        }

        // Display search results
        val adapter = VerseResultAdapter()
        binding.resultsRecycler.adapter = adapter

        viewModel.results.observe(viewLifecycleOwner, Observer {
            it?.let {
                adapter.data = it.resultsList
            }
        })

        // Done!
        return binding.root
    }
}
