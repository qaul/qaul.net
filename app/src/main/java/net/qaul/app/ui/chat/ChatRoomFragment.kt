package net.qaul.app.ui.chat

import android.app.ActionBar
import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import net.qaul.app.R
import net.qaul.app.ffi.models.ChatRoom


class ChatRoomFragment(val room: ChatRoom) : Fragment() {

    override fun onCreateView(inflater: LayoutInflater, container: ViewGroup?, bundle: Bundle?): View? {
        val root = inflater.inflate(R.layout.fragment_chatroom, container, false)

        // TODO: add a back button maybe?

        return root
    }
}
