package net.qaul.app.ui

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.Button
import android.widget.EditText
import android.widget.Toast
import androidx.fragment.app.Fragment
import net.qaul.app.LoginActivity
import net.qaul.app.R
import net.qaul.app.util.AppState

class UserCreateFragment(val login: LoginActivity) : Fragment() {
    override fun onCreateView(inflater: LayoutInflater, container: ViewGroup?, bundle: Bundle?): View? {
        val root = inflater.inflate(R.layout.fragment_register, container, false)

        val password = root.findViewById<EditText>(R.id.registry_password_entry)
        val handle = root.findViewById<EditText>(R.id.registry_handle)
        val name = root.findViewById<EditText>(R.id.registry_name)

        val button = root.findViewById<Button>(R.id.registry_create)
        button.setOnClickListener {
            val id = AppState.get().usersCreate(handle.text.toString(), name.text.toString(), password.text.toString())
            Toast.makeText(context, "Your user ID is: '${id.inner}'", Toast.LENGTH_LONG).show()

            login.updateUsers()
            parentFragmentManager.popBackStack() // Go back to login!
        }

        return root
    }
}