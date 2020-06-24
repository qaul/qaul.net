package net.qaul.app.ui.chat

import android.content.Context
import android.util.Log
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.FragmentManager
import androidx.fragment.app.FragmentTransaction
import androidx.recyclerview.widget.RecyclerView
import kotlinx.android.synthetic.main.item_chat_room.view.*
import net.qaul.app.R
import net.qaul.app.ffi.models.ChatRoom
import net.qaul.app.util.inflate

class ChatListAdapter(private val fragMan: FragmentManager, private val rooms: MutableList<ChatRoom>)
    : RecyclerView.Adapter<ChatListAdapter.RoomHolder>() {

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): RoomHolder {
        val inflated = parent.inflate(R.layout.item_chat_room, false)
        return RoomHolder( inflated, fragMan)
    }

    override fun getItemCount() = rooms.size

    override fun onBindViewHolder(holder: RoomHolder, position: Int) {
        holder.bindRoom(rooms[position])
    }

    class RoomHolder(v: View, val fragMan: FragmentManager) : RecyclerView.ViewHolder(v), View.OnClickListener {
        private var view: View = v
        var room: ChatRoom? = null

        init {
            v.setOnClickListener(this)
        }

        fun bindRoom(room: ChatRoom) {
            this.room = room

            // Then set the UI state
            view.chatroom_list_item_name.text = room.name!!
            view.chatroom_list_item_timestamp.text = room.last_message!!
            view.chatroom_list_item_unread_count.text = room.unread.toString()
        }

        override fun onClick(v: View?) {
            Log.d("ChatRooms", "Selecting room: " + room!!.name)

            val fragTrans = fragMan.beginTransaction()
            val chatFrag = ChatRoomFragment(room!!)
            fragTrans.replace(R.id.nav_host_fragment, chatFrag)
            fragTrans.setTransition( FragmentTransaction.TRANSIT_FRAGMENT_OPEN )
            fragTrans.commit()
        }

        companion object {
            private const val ROOM_KEY = "ROOM"
        }
    }
}
