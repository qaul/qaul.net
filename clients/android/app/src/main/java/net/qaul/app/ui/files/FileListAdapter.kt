package net.qaul.app.ui.files

import android.net.Network
import android.util.Log
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import androidx.fragment.app.FragmentManager
import androidx.fragment.app.FragmentStatePagerAdapter
import androidx.fragment.app.FragmentTransaction
import androidx.recyclerview.widget.RecyclerView
import kotlinx.android.synthetic.main.item_chat_room.view.*
import net.qaul.app.R
import net.qaul.app.ffi.models.ChatRoom
import net.qaul.app.ffi.models.NetworkFile
import net.qaul.app.util.inflate

class FileListAdapter(private val fragMan: FragmentManager)
    : RecyclerView.Adapter<FileListAdapter.FileHolder>() {

    val files: MutableList<NetworkFile> = mutableListOf()

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): FileHolder {
        val inflated = parent.inflate(R.layout.item_chat_room, false)
        return FileHolder(inflated, fragMan)
    }

    override fun getItemCount() = files.size

    override fun onBindViewHolder(holder: FileHolder, position: Int) {
        holder.bindFile(files[position])
    }

    class FileHolder(v: View, private val man: FragmentManager)
        : RecyclerView.ViewHolder(v), View.OnClickListener {
        private var view: View = v
        var file: NetworkFile? = null

        init { v.setOnClickListener(this) }

        fun bindFile(file: NetworkFile) {
            this.file = file

        }

        override fun onClick(v: View?) {
//            val fragment = ChatRoomFragment(room!!)
//
//            // Create transaction to replace main container view
//            val trans = man.beginTransaction()
//            trans.replace(R.id.nav_host_fragment, fragment).addToBackStack(null)
//            trans.setTransition(FragmentTransaction.TRANSIT_FRAGMENT_OPEN)
//            trans.commit()
        }

    }
}
