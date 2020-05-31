package net.qaul.app.ui.chat

import android.content.Context
import android.util.Log
import android.view.View
import android.view.ViewGroup
import androidx.recyclerview.widget.RecyclerView
import kotlinx.android.synthetic.main.item_chat_room.view.*
import net.qaul.app.R
import net.qaul.app.ffi.models.ChatRoom
import net.qaul.app.util.inflate

class ChatListAdapter(private val rooms: ArrayList<ChatRoom>)
    : RecyclerView.Adapter<ChatListAdapter.ViewHolder>() {

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): ViewHolder {
        val inflated = parent.inflate(R.layout.item_chat_room, false)
        return ViewHolder(inflated)
    }

    override fun getItemCount() = rooms.size

    override fun onBindViewHolder(holder: ViewHolder, position: Int) {
        val room = rooms[position]
        holder.room = room
    }

    class ViewHolder(v: View) : RecyclerView.ViewHolder(v), View.OnClickListener {
        private var view: View = v
        var room: ChatRoom? = null

        init {
            v.setOnClickListener(this)
        }

        fun bind(room: ChatRoom) {
            view.item_name.text = room.name!!
            view.item_timestamp.text = room.last_message!!
            view.item_unread_count.text = room.unread.toString()
        }

        override fun onClick(v: View?) {
            Log.d("ChatRooms", "Selecting room: " + room!!.name)
        }

        companion object {
            private val ROOM_KEY = "ROOM"
        }
    }
}