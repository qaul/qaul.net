package net.qaul.app.ffi.models;

public class UserProfile {
    public String id;
    public String displayName;
    public String realName;

    public UserProfile(String id, String displayName, String realName) {
            this.id = id;
            this.displayName = displayName;
            this.realName = realName;
    }
}
