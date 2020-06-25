package net.qaul.app.ui.settings

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.EditText
import androidx.fragment.app.Fragment
import net.qaul.app.R
import net.qaul.app.ffi.models.UserProfile

class SettingsFragment : Fragment() {

    override fun onCreateView(inflater: LayoutInflater, container: ViewGroup?, bundle: Bundle?): View? {
        val root = inflater.inflate(R.layout.fragment_settings, container, false)
        val self = UserProfile("", "@spacekookie", "Katharina Fey", false)

        val handle = root.findViewById<EditText>(R.id.user_profile_set_handle)
        handle.setText(self.displayName)

        val name = root.findViewById<EditText>(R.id.user_profile_set_name)
        name.setText(self.realName)

        return root
    }
}
