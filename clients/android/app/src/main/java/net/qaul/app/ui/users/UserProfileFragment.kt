package net.qaul.app.ui.users

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.Button
import android.widget.ImageView
import android.widget.TextView
import androidx.fragment.app.Fragment
import net.qaul.app.R
import net.qaul.app.ffi.models.Id
import net.qaul.app.ffi.models.UserProfile
import net.qaul.app.ui.chat.ChatRoomFragment
import net.qaul.app.util.AppState
import java.util.ArrayList

class UserProfileFragment(val profile: UserProfile) : Fragment() {
    override fun onCreateView(inflater: LayoutInflater, container: ViewGroup?, savedInstanceState: Bundle?): View? {
        val root = inflater.inflate(R.layout.fragment_user_profile, container, false)

        val name = root.findViewById<TextView>(R.id.user_profile_name)
        val handle = root.findViewById<TextView>(R.id.user_profile_handle)
        val lastOnline = root.findViewById<TextView>(R.id.user_profile_last_online)

        val markFriend = root.findViewById<Button>(R.id.user_profile_mark_friend)
        markFriend.setOnClickListener {
            profile.friend = !profile.friend
            update_friend_status(root, markFriend)
        }

        update_friend_status(root, markFriend)

        val avi = root.findViewById<ImageView>(R.id.user_profile_avi)
        avi.setVisibility(View.VISIBLE)

        name.text = profile.name
        handle.text = profile.handle
        lastOnline.text = "now"

        // Setup button listeners
        val startChat = root.findViewById<Button>(R.id.user_profile_open_chat)
        startChat.setOnClickListener {
            val room = AppState.get().chatStart(profile.handle, listOf<Id>(profile.id) as ArrayList<Id>?)
            ChatRoomFragment(room).transitionInto(parentFragmentManager)
        }

        return root
    }

    fun update_friend_status(root: View, markFriend: Button) {
        val friend = root.findViewById<ImageView>(R.id.user_profile_friend_status)

        friend.setVisibility(if(profile.friend) {
            markFriend.text = "Unfriend"
            View.VISIBLE
        } else {
            markFriend.text = "Mark friend"
            View.INVISIBLE
        })


    }
}