package net.qaul.app.ui.voice

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import net.qaul.app.R

class VoiceFragment : Fragment() {

    override fun onCreateView(inflater: LayoutInflater, container: ViewGroup?, bundle: Bundle?): View? {
        val root = inflater.inflate(R.layout.fragment_voice, container, false)

        return root
    }
}
