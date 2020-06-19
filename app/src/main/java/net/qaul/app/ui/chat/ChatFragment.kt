package net.qaul.app.ui.chat

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import androidx.lifecycle.ViewModelProviders
import androidx.recyclerview.widget.LinearLayoutManager
import androidx.recyclerview.widget.RecyclerView
import kotlinx.android.synthetic.main.fragment_chat.*
import net.qaul.app.R
import net.qaul.app.ffi.models.ChatRoom

class ChatFragment : Fragment() {

    // He's the layer-outer
    private lateinit var layouter: LinearLayoutManager
    private lateinit var adapter: ChatListAdapter
    private lateinit var chatFragment: ChatViewModel

    override fun onCreateView(
            inflater: LayoutInflater,
            container: ViewGroup?,
            savedInstanceState: Bundle?
    ): View? {
        chatFragment = ViewModelProviders.of(this).get(ChatViewModel::class.java)
        val root = inflater.inflate(R.layout.fragment_chat, container, false)
        layouter = LinearLayoutManager(context)

        val list = view?.findViewById<RecyclerView>(R.id.chat_room_list)
        list!!.layoutManager = layouter
        
        val rooms = listOf<ChatRoom>(ChatRoom("id1", "Alice Anonymous", "2020-05-31 13:12", 5, ArrayList()),
                ChatRoom("id2", "Caren Cop", "2008-01-01 00:33", 0, ArrayList()),
                ChatRoom("id3", "Danni Default", "2020-05-31 13:37", 2, ArrayList()))

        adapter = ChatListAdapter(rooms as ArrayList<ChatRoom>)
        chat_room_list.adapter = adapter


        return root
    }
}
