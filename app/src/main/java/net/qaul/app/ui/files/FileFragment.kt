package net.qaul.app.ui.files

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.TextView
import androidx.fragment.app.Fragment
import androidx.lifecycle.Observer
import androidx.lifecycle.ViewModelProviders
import net.qaul.app.R

class FileFragment : Fragment() {

    private lateinit var fileFragment: FileViewModel

    override fun onCreateView(
            inflater: LayoutInflater,
            container: ViewGroup?,
            savedInstanceState: Bundle?
    ): View? {
        fileFragment = ViewModelProviders.of(this).get(FileViewModel::class.java)
        val root = inflater.inflate(R.layout.fragment_files, container, false)
        val textView: TextView = root.findViewById(R.id.text_files)
        fileFragment.text.observe(viewLifecycleOwner, Observer {
            textView.text = it
        })
        return root
    }
}
