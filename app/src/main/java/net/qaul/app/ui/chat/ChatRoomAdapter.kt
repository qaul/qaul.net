package net.qaul.app.ui.chat

import android.content.Context
import android.util.Log
import android.view.View
import android.view.ViewGroup
import androidx.recyclerview.widget.RecyclerView
import kotlinx.android.synthetic.main.item_chat_message_received.view.*
import kotlinx.android.synthetic.main.item_chat_message_sent.view.*
import kotlinx.android.synthetic.main.item_chat_message_sent.view.chat_message_body
import kotlinx.android.synthetic.main.item_chat_message_sent.view.chat_message_time
import net.qaul.app.R
import net.qaul.app.ffi.models.ChatMessage
import net.qaul.app.ffi.models.UserProfile
import net.qaul.app.util.inflate
import java.lang.Exception

const val MESSAGE_TYPE_SENT: Int = 1
const val MESSAGE_TYPE_RECV: Int = 2

class ChatRoomAdapter(private val self: UserProfile, private val messages: MutableList<ChatMessage>)
    : RecyclerView.Adapter<ChatRoomAdapter.MsgHolder>() {

    override fun getItemViewType(position: Int): Int {
        if (messages[position].author == self.displayName) {
            return MESSAGE_TYPE_SENT
        } else {
            return MESSAGE_TYPE_RECV
        }
    }

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): MsgHolder {
        when (viewType){
            MESSAGE_TYPE_SENT -> {
                val inflated = parent.inflate(R.layout.item_chat_message_sent)
                return MsgHolder(inflated, false)
            }
            MESSAGE_TYPE_RECV -> {
                val inflated = parent.inflate(R.layout.item_chat_message_received)
                return MsgHolder(inflated, true)
            }
            else -> throw Exception("Invalid ViewType for message!  Not 1 or 2")
        }
    }

    override fun getItemCount() = messages.size

    override fun onBindViewHolder(holder: MsgHolder, position: Int) {
        holder.bindMessage(messages[position])
    }

    class MsgHolder(v: View, private val received: Boolean)
        : RecyclerView.ViewHolder(v), View.OnClickListener {
        private var view: View = v
        lateinit var msg: ChatMessage

        init {
            v.setOnClickListener(this)
        }

        fun bindMessage(msg: ChatMessage) {
            this.msg = msg

            view.chat_message_body.text = msg.content!!
            view.chat_message_time.text = msg.timestamp!!

            if (received) {
                view.chat_message_name.text = msg.author!!
            }
        }

        override fun onClick(v: View?) {
            Log.d("chatroom", "We might want to do something with long-presses")

        }

        companion object {
            private const val ROOM_KEY = "ROOM"
        }
    }
}
