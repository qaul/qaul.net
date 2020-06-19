package net.qaul.app.ui.settings

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.TextView
import androidx.fragment.app.Fragment
import androidx.lifecycle.Observer
import androidx.lifecycle.ViewModelProviders
import net.qaul.app.R

class SettingsFragment : Fragment() {

    private lateinit var settingsFragment: SettingsViewModel

    override fun onCreateView(
            inflater: LayoutInflater,
            container: ViewGroup?,
            savedInstanceState: Bundle?
    ): View? {
        settingsFragment = ViewModelProviders.of(this).get(SettingsViewModel::class.java)
        val root = inflater.inflate(R.layout.fragment_files, container, false)
        val textView: TextView = root.findViewById(R.id.text_files)
        settingsFragment.text.observe(viewLifecycleOwner, Observer {
            textView.text = it
        })
        return root
    }
}
