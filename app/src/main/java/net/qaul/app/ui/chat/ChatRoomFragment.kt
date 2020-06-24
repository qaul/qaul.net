package net.qaul.app.ui.chat

import android.app.ActionBar
import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import androidx.recyclerview.widget.LinearLayoutManager
import androidx.recyclerview.widget.RecyclerView
import net.qaul.app.R
import net.qaul.app.ffi.models.ChatMessage
import net.qaul.app.ffi.models.ChatRoom
import net.qaul.app.ffi.models.UserProfile


class ChatRoomFragment(val room: ChatRoom) : Fragment() {
    private lateinit var layouter: LinearLayoutManager
    private lateinit var adapter: ChatRoomAdapter

    override fun onCreateView(inflater: LayoutInflater, container: ViewGroup?, bundle: Bundle?): View? {
        val root = inflater.inflate(R.layout.fragment_chatroom, container, false)
        layouter = LinearLayoutManager(context)

        // TODO: add a back button maybe?

        // Some messages
        val messages: MutableList<ChatMessage> = mutableListOf(
                ChatMessage("", "15:11", "Hey, how are you?", "alice"),
                ChatMessage("", "15:32", "Not bad, kinda stressed", "spacekookie"),
                ChatMessage("", "15:33", "Trying to get this app to work", "spacekookie"),
                ChatMessage("", "15:36", "Yea? What's the problem?", "alice"),
                ChatMessage("", "15:41", "There's just so many things that don't work properly and Android has the tendency to layer lots of abstractions on top of each other, and trying to get them all to play nice is really annoying." +
                        "" +
                        "Really, I wish I could just not do any of this >.>", "spacekookie")
        )

        val self = UserProfile("", "spacekookie", "Katharina Fey")

        adapter = ChatRoomAdapter(self, messages)
        val chatRoomList = root.findViewById<RecyclerView>(R.id.chatroom_message_list)
        chatRoomList.adapter = adapter
        chatRoomList.layoutManager = LinearLayoutManager(context)

        return root
    }
}
