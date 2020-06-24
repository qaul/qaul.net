package net.qaul.app.ui.chat

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import androidx.fragment.app.FragmentManager
import androidx.recyclerview.widget.LinearLayoutManager
import androidx.recyclerview.widget.RecyclerView
import com.google.android.material.floatingactionbutton.FloatingActionButton
import net.qaul.app.R
import net.qaul.app.ffi.models.ChatRoom
import net.qaul.app.util.defanSubFabs
import net.qaul.app.util.fanSubFabs
import net.qaul.app.util.rotateFab

class ChatFragment : Fragment() {

    // He's the layer-outer
    private lateinit var layouter: LinearLayoutManager
    private lateinit var adapter: ChatListAdapter

    var fabRotated: Boolean = false
    var originFab: Float = 0.0f

    lateinit var fragMan: FragmentManager

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View? {
        val root = inflater.inflate(R.layout.fragment_chat, container, false)
        layouter = LinearLayoutManager(context)
        fragMan = parentFragmentManager

        val list = root!!.findViewById<RecyclerView>(R.id.chat_room_list)!!
        list.layoutManager = layouter

        val rooms: MutableList<ChatRoom> = mutableListOf(
                ChatRoom("id1", "Alice Anonymous", "2020-05-31 13:12", 5, ArrayList()),
                ChatRoom("id2", "Caren Cop", "2008-01-01 00:33", 0, ArrayList()),
                ChatRoom("id3", "Danni Default", "2020-05-31 13:37", 2, ArrayList())
        )

        adapter = ChatListAdapter(rooms, fragMan)
        val chatRoomList = root.findViewById<RecyclerView>(R.id.chat_room_list)
        chatRoomList.adapter = adapter
        chatRoomList.layoutManager = LinearLayoutManager(context)

        // Do the FAB stuff
        val fab = root.findViewById<FloatingActionButton>(R.id.chat_room_list_start)
        val fab_single = root.findViewById<FloatingActionButton>(R.id.chat_room_list_start_chat)
        val fab_group = root.findViewById<FloatingActionButton>(R.id.chat_room_list_start_group)

        originFab = fab_single.y

        fab.setOnClickListener {
            fabRotated = !fabRotated
            rotateFab(fab, fabRotated)

            if(fabRotated) {
                val yOffset = ((fab.height - fab_single.height) / 2) + fab_single.height
                fanSubFabs(listOf(fab_single, fab_group), 15, yOffset)
            } else {
                defanSubFabs(listOf(fab_single, fab_group), originFab)
            }
        }

        fab_single.setOnClickListener {
            // Select a user
        }

        fab_group.setOnClickListener {
            // Select multiple users
        }

        return root
    }
}
