package net.qaul.app.ui.users

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.TextView
import androidx.fragment.app.Fragment
import androidx.lifecycle.Observer
import androidx.lifecycle.ViewModelProviders
import net.qaul.app.R
import net.qaul.app.ui.files.FileViewModel

class UsersFragment : Fragment() {
    private lateinit var usersFragment: UsersViewModel

    override fun onCreateView(inflater: LayoutInflater,
                              container: ViewGroup?,
                              savedInstanceState: Bundle?): View? {
        usersFragment = ViewModelProviders.of(this).get(UsersViewModel::class.java)
        val root = inflater.inflate(R.layout.fragment_files, container, false)
        val textView: TextView = root.findViewById(R.id.text_files)
        usersFragment.text.observe(viewLifecycleOwner, Observer {
            textView.text = it
        })
        return root
    }
}
