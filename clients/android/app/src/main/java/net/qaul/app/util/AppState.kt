package net.qaul.app.util

import net.qaul.app.ffi.NativeQaul
import net.qaul.app.ffi.models.Id
import net.qaul.app.ffi.models.UserProfile

object AppState {
    lateinit var self: UserProfile
    // val libqaul: NativeQaul = NativeQaul(0, "") // FIXME: remove params

    private val usersCache: MutableMap<Id, UserProfile> = mutableMapOf<Id, UserProfile>()

    /**
     * Resolve a user ID to the user profile.  Fetch from Rust code if not in cache
     */
    fun getUserProfile(id: Id): UserProfile? {
        return this.usersCache[id]
    }
}