package net.qaul.app.ui.chat

import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel


class ChatViewModel : ViewModel() {

    private val _text = MutableLiveData<String>().apply {
        value = "This is chat Fragment"
    }
    val text: LiveData<String> = _text
}