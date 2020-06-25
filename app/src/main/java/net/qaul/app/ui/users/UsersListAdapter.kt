package net.qaul.app.ui.users

import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.FragmentManager
import androidx.fragment.app.FragmentTransaction
import androidx.recyclerview.widget.RecyclerView
import kotlinx.android.synthetic.main.item_users_list.view.*
import net.qaul.app.R
import net.qaul.app.ffi.models.UserProfile
import net.qaul.app.util.inflate


open class UsersListAdapter(private val rooms: MutableList<UserProfile>, private val fragMan: FragmentManager)
    : RecyclerView.Adapter<UsersListAdapter.ProfileHolder>() {

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): ProfileHolder {
        val inflated = parent.inflate(R.layout.item_users_list, false)
        return ProfileHolder(inflated, fragMan)
    }

    override fun getItemCount() = rooms.size

    override fun onBindViewHolder(holder: ProfileHolder, position: Int) {
        holder.bindUser(rooms[position])
    }

    class ProfileHolder(v: View, private val man: FragmentManager)
        : RecyclerView.ViewHolder(v), View.OnClickListener {
        private var view: View = v
        var profile: UserProfile? = null

        init {
            v.setOnClickListener(this)
        }

        fun bindUser(profile: UserProfile) {
            this.profile = profile

            view.item_users_list_name.text = profile.realName
            view.item_users_list_online.text = profile.displayName
        }

        override fun onClick(v: View?) {
            val fragment = UserProfileFragment(profile!!)

            // Create transaction to replace main container view
            val trans = man.beginTransaction()
            trans.replace(R.id.nav_host_fragment, fragment).addToBackStack(null)
            trans.setTransition(FragmentTransaction.TRANSIT_FRAGMENT_OPEN)
            trans.commit()
        }
    }
}
