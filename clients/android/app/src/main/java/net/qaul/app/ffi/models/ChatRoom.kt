package net.qaul.app.ffi.models

data class ChatRoom(val id: String?,
                    val name: String?,
                    val last_message: String?,
                    val unread: Int,
                    val members: ArrayList<String>)