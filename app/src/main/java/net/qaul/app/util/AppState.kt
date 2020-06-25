package net.qaul.app.util

import net.qaul.app.ffi.NativeQaul
import net.qaul.app.ffi.models.UserProfile

object AppState {
    lateinit var self: UserProfile
    // val libqaul: NativeQaul = NativeQaul(0, "") // FIXME: remove params
}