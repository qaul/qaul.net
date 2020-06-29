package net.qaul.app.ffi.models;

import java.util.ArrayList;

/**
 * A chat room with either two or more people, talking about crimes¹
 *
 * ¹ talking about crimes is not required
 */
public class ChatRoom {
    public Id id;
    public String name;
    public String last_message;
    public int unread;
    public ArrayList<Id> members;

    public ChatRoom(Id id, String name, String last_message, int unread, ArrayList<Id> members) {
        this.id = id;
        this.name = name;
        this.last_message = last_message;
        this.unread = unread;
        this.members = members;
    }
}
