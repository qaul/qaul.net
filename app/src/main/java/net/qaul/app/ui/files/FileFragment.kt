package net.qaul.app.ui.files

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import androidx.fragment.app.FragmentManager
import androidx.recyclerview.widget.LinearLayoutManager
import androidx.recyclerview.widget.RecyclerView
import net.qaul.app.R

class FileFragment : Fragment() {
    private lateinit var adapter: FileListAdapter
    private lateinit var fragMan: FragmentManager
    private lateinit var layouter: LinearLayoutManager

    override fun onCreateView(inflater: LayoutInflater, container: ViewGroup?, bundle: Bundle?): View? {
        val root = inflater.inflate(R.layout.fragment_files, container, false)
        layouter = LinearLayoutManager(context)
        fragMan = parentFragmentManager
        adapter = FileListAdapter(fragMan)

        val list = root!!.findViewById<RecyclerView>(R.id.file_list)!!
        list.layoutManager = layouter
        list.adapter = adapter

        return root
    }
}
