// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

package net.qaul.ble.callback

import com.google.protobuf.ByteString

interface BleRequestCallback {
    fun bleResponse(data: ByteString)
}