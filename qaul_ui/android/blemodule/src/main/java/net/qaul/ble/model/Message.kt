package net.qaul.ble.model

import com.google.gson.annotations.SerializedName

data class Message(

    @field:SerializedName("qaul_id")
    var qaulId: ByteArray? = null,

    @field:SerializedName("message")
    var message: ByteArray? = null
)
