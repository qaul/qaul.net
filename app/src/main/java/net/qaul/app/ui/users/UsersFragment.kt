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

class UsersFragment : Fragment() {
//    private lateinit var usersFragment: UsersViewModel

    override fun onCreateView(inflater: LayoutInflater, container: ViewGroup?, bundle: Bundle?): View? {
        return inflater.inflate(R.layout.fragment_users, container, false)


//        usersFragment = ViewModelProviders.of(this).get(UsersViewModel::class.java)
//        val textView: TextView = root.findViewById(R.id.text_files)

//        usersFragment.text.observe(viewLifecycleOwner, Observer {
//            textView.text = it
//        })

    }
}
