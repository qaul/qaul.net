package net.qaul.app.ffi.models;

/**
 * Represent a user profile with ID and metadata
 */
public class UserProfile {
    public Id id;
    public String handle;
    public String name;
    public boolean friend;

    public UserProfile(Id id, String handle, String name, boolean friend) {
            this.id = id;
            this.handle = handle;
            this.name = name;
            this.friend = friend;
    }
}
