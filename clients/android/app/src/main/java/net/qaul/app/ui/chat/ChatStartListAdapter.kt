package net.qaul.app.ui.chat

import android.view.View
import android.view.ViewGroup
import android.widget.ImageView
import androidx.fragment.app.FragmentManager
import androidx.fragment.app.FragmentTransaction
import androidx.recyclerview.widget.RecyclerView
import kotlinx.android.synthetic.main.item_chat_room.view.*
import kotlinx.android.synthetic.main.item_users_list.view.*
import net.qaul.app.R
import net.qaul.app.ffi.models.ChatRoom
import net.qaul.app.ffi.models.UserProfile
import net.qaul.app.util.inflate

class ChatStartListAdapter(private val fragMan: FragmentManager)
    : RecyclerView.Adapter<ChatStartListAdapter.ViewHolder>() {

    val users: MutableList<UserProfile> = mutableListOf()

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): ViewHolder {
        val inflated = parent.inflate(R.layout.item_chat_room, false)
        return ViewHolder(inflated, fragMan)
    }

    override fun getItemCount() = users.size

    override fun onBindViewHolder(holder: ViewHolder, position: Int) {
        holder.bind(users[position])
    }

    class ViewHolder(v: View, private val man: FragmentManager)
        : RecyclerView.ViewHolder(v), View.OnClickListener {
        private var view: View = v
        var profile: UserProfile? = null

        init {
            v.setOnClickListener(this)
        }

        fun bind(profile: UserProfile) {
            this.profile = profile

            view.item_users_list_name.text = profile.name
            view.item_users_list_online.text = profile.handle
        }

        fun getState(): Boolean {
            val check = view.findViewById<ImageView>(R.id.user_selected_check)
            return check.visibility == View.VISIBLE
        }

        override fun onClick(v: View?) {
            val check = view.findViewById<ImageView>(R.id.user_selected_check)
            if (getState()) {
                check.visibility = View.VISIBLE
            } else {
                check.visibility = View.INVISIBLE
            }
        }
    }
}
