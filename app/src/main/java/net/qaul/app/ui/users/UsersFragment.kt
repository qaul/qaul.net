package net.qaul.app.ui.users

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import androidx.fragment.app.FragmentManager
import androidx.fragment.app.FragmentStatePagerAdapter
import androidx.recyclerview.widget.LinearLayoutManager
import androidx.recyclerview.widget.RecyclerView
import androidx.viewpager.widget.ViewPager
import com.google.android.material.tabs.TabLayout
import net.qaul.app.R
import net.qaul.app.ffi.models.UserProfile

class UsersFragment : Fragment() {

    private lateinit var adapter: UsersTabsAdapter
    private lateinit var pager: ViewPager


    override fun onCreateView(inflater: LayoutInflater, container: ViewGroup?, bundle: Bundle?): View? {
        return inflater.inflate(R.layout.fragment_users, container, false)
    }

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        adapter = UsersTabsAdapter(parentFragmentManager)
        pager = view.findViewById(R.id.users_tab_pager)
        pager.adapter = adapter

        val layout = view.findViewById<TabLayout>(R.id.users_tab_layout)
        layout.setupWithViewPager(pager)
    }
}


class UsersTabsAdapter(val fm: FragmentManager)
    : FragmentStatePagerAdapter(fm, BEHAVIOR_RESUME_ONLY_CURRENT_FRAGMENT) {
    override fun getItem(position: Int): Fragment {
        if (position == 1) {
            return UsersListFragment(mutableListOf<UserProfile>(
                    UserProfile("", "@carencop","Caren Cop", false),
                    UserProfile("", "@bob","Bob Beligerant", false),
                    UserProfile("", "@eve","Eve Evergreen", false)
            ), fm)
        } else {
            return UsersListFragment(mutableListOf<UserProfile>(
                    UserProfile("", "danni","Danni Default", true),
                    UserProfile("", "alice","Alice Anonymous", true)
            ), fm)
        }
    }

    override fun getPageTitle(position: Int): CharSequence? {
        if(position == 1) {
            return "All Users"
        } else {
            return "Friends"
        }
    }

    override fun getCount(): Int = 2

}

class UsersListFragment(private val users: MutableList<UserProfile>, val fm: FragmentManager) : Fragment() {
    private lateinit var adapter: UsersListAdapter
    private lateinit var layouter: LinearLayoutManager

    override fun onCreateView(inflater: LayoutInflater, container: ViewGroup?, savedInstanceState: Bundle?): View? {
        val root = inflater.inflate(R.layout.fragment_users_list, container, false)
        adapter = UsersListAdapter(users, fm)
        layouter = LinearLayoutManager(context)

        val list = root!!.findViewById<RecyclerView>(R.id.users_list)!!
        list.adapter = adapter
        list.layoutManager = layouter

        return root
    }
}
