package net.qaul.app.ffi.models;

public class UserProfile {
    public String id;
    public String displayName;
    public String realName;
    public boolean friend;

    public UserProfile(String id, String displayName, String realName, boolean friend) {
        this.id = id;
        this.displayName = displayName;
        this.realName = realName;
        this.friend = friend;
    }
}
