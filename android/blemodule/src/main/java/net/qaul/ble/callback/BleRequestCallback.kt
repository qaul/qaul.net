// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

package net.qaul.ble.callback

import qaul.sys.ble.BleOuterClass

interface BleRequestCallback {
    fun bleResponse(ble: BleOuterClass.Ble)
}