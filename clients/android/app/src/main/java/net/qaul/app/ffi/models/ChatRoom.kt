package net.qaul.app.ffi.models

data class ChatRoom(val id: String?,
                    val name: String?,
                    val unread: Int,
                    val members: ArrayList<String>)