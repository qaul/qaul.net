package net.qaul.app.util

import net.qaul.app.ffi.NativeQaul
import net.qaul.app.ffi.models.Id
import net.qaul.app.ffi.models.UserProfile

object AppState {
    lateinit var self: UserProfile
    private var libqaul: NativeQaul? = null

    private val usersCache: MutableMap<Id, UserProfile> = mutableMapOf<Id, UserProfile>()

    /**
     * Get the current native state and initialise it first
     */
    fun get(): NativeQaul = if (libqaul != null) {
        libqaul!!
    } else {
        libqaul = NativeQaul(0)
        libqaul!!
    }

    /**
     * Resolve a user ID to the user profile.  Fetch from Rust code if not in cache
     */
    fun getUserProfile(id: Id): UserProfile? {
        return this.usersCache[id]
    }
}