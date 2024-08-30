// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

package net.qaul.ble.model

import com.google.gson.annotations.SerializedName

    data class Message(

        @field:SerializedName("qaul_id")
        var qaulId: ByteArray? = null,

        @field:SerializedName("message")
        var message: ByteArray? = null
    )
