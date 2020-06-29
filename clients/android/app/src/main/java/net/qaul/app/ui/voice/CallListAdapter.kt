package net.qaul.app.ui.voice

import android.telecom.Call
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.FragmentManager
import androidx.recyclerview.widget.RecyclerView
import net.qaul.app.R
import net.qaul.app.util.inflate

class CallListAdapter(private val rooms: MutableList<Call>, private val fragMan: FragmentManager)
    : RecyclerView.Adapter<CallListAdapter.CallHolder>() {

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): CallHolder {
        val inflated = parent.inflate(R.layout.item_chat_room, false)
        return CallHolder(inflated, fragMan)
    }

    override fun getItemCount() = rooms.size

    override fun onBindViewHolder(holder: CallHolder, position: Int) {
        holder.bind(rooms[position])
    }

    class CallHolder(v: View, private val man: FragmentManager)
        : RecyclerView.ViewHolder(v), View.OnClickListener {
        private var view: View = v
        var call: Call? = null

        init { v.setOnClickListener(this) }

        fun bind(call: Call) {
            this.call = call
        }

        override fun onClick(v: View?) {}
    }
}
