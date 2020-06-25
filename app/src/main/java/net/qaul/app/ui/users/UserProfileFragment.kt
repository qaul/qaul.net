package net.qaul.app.ui.users

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.ImageView
import android.widget.TextView
import androidx.fragment.app.Fragment
import net.qaul.app.R
import net.qaul.app.ffi.models.UserProfile

class UserProfileFragment(val profile: UserProfile) : Fragment() {
    override fun onCreateView(inflater: LayoutInflater, container: ViewGroup?, savedInstanceState: Bundle?): View? {
        val root = inflater.inflate(R.layout.fragment_user_profile, container, false)

        val name = root.findViewById<TextView>(R.id.user_profile_name)
        val handle = root.findViewById<TextView>(R.id.user_profile_handle)
        val lastOnline = root.findViewById<TextView>(R.id.user_profile_last_online)

        val friend = root.findViewById<ImageView>(R.id.user_profile_friend_status)
        friend.setVisibility(if(profile.friend) {
            View.VISIBLE
        } else {
            View.INVISIBLE
        })

        val avi = root.findViewById<ImageView>(R.id.user_profile_avi)
        avi.setVisibility(View.VISIBLE)

        name.text = profile.realName
        handle.text = profile.displayName
        lastOnline.text = "now"


        return root
    }
}