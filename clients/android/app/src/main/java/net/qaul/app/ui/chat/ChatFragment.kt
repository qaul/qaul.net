package net.qaul.app.ui.chat

import android.annotation.SuppressLint
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
import net.qaul.app.ffi.models.Id
import net.qaul.app.util.AppState
import net.qaul.app.util.defanSubFabs
import net.qaul.app.util.fanSubFabs
import net.qaul.app.util.rotateFab

class ChatFragment : Fragment() {

    var fabRotated: Boolean = false
    var originFab: Float = 0.0f

    lateinit var fragMan: FragmentManager

    val chatRooms: MutableList<ChatRoom> = mutableListOf()
    lateinit var chatList: RecyclerView

    fun updateRooms() {
        chatRooms.clear()
        for(r in AppState.get().chatList()) {
            chatRooms.add(r)
        }

        chatList.adapter = ChatListAdapter(chatRooms, fragMan)
    }

    @SuppressLint("RestrictedApi")
    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View? {
        val root = inflater.inflate(R.layout.fragment_chat, container, false)
        fragMan = parentFragmentManager

        val list = root!!.findViewById<RecyclerView>(R.id.chat_room_list)!!
        list.layoutManager = LinearLayoutManager(context)

        chatList = root.findViewById<RecyclerView>(R.id.chat_room_list)
        chatList.layoutManager = LinearLayoutManager(context)
        updateRooms()

        // Do the FAB stuff
        val fab = root.findViewById<FloatingActionButton>(R.id.chat_room_list_start)
        val fab_single = root.findViewById<FloatingActionButton>(R.id.chat_room_list_start_chat)
        val fab_group = root.findViewById<FloatingActionButton>(R.id.chat_room_list_start_group)

        fab.visibility = View.INVISIBLE
        fab_single.visibility = View.INVISIBLE
        fab_group.visibility = View.INVISIBLE

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
