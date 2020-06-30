package net.qaul.app.ffi;

import android.view.View;

import net.qaul.app.ffi.models.ChatMessage;
import net.qaul.app.ffi.models.ChatRoom;
import net.qaul.app.ffi.models.Frame;
import net.qaul.app.ffi.models.Id;
import net.qaul.app.ffi.models.UserProfile;

import java.util.ArrayList;

/**
 * The native libqaul bridge interface.
 * <p>
 * This file/class is written in Java because FFI integration between Kotlin and Rust
 * might be more complicated than with Java (for example javah exists, where there
 * doesn't seem to be a comparable kotlinh).  This can be changed in the future, and
 * this should definitely remain the only Java code, but this is simpler for now.
 */
public class NativeQaul {
    private long libqaulState = 0;

    public NativeQaul(int port) {
        this.libqaulState = setupState(port);
    }

    public native Id idTest(Id id);

    /**
     * Setup the main
     */
    private native long setupState(int port);

    /**
     * Start peering the TCP endpoint to a particular server
     *
     * @param addr the remote server address
     * @param port the remote server port
     */
    public void connectTpc(String addr, int port) {
        connectTcp(libqaulState, addr, port);
    }

    private native void connectTcp(long qaul, String addr, int port);

    /**
     * Check if the instance has a valid login
     *
     * @return true if login is valid
     */
    private native boolean checkLogin(long qaul);

    /**
     * List available users
     *
     * @local indicate whether only to list local users
     *
     * @return List of local users
     */
    public ArrayList<UserProfile> usersList(boolean local) {
        return usersList(libqaulState, local);
    }

    private native ArrayList<UserProfile> usersList(long qaul, boolean local);

    /**
     * Create a new user
     *
     */
    public Id usersCreate(String handle, String name, String password) {
        return usersCreate(libqaulState, handle, name, password);
    }

    private native Id usersCreate(long qaul, String handle, String name, String password);

    /**
     * Get a particular user profile by ID
     *
     * @return List of local users
     */
    public UserProfile usersGet(Id id) {
        return usersGet(libqaulState, id);
    }

    private native UserProfile usersGet(long qaul, Id id);

    /**
     * Modify the local user profile and return the new data
     *
     * @return Modified user profile
     */
    public UserProfile usersModify(String handle, String name) {
        return usersModify(libqaulState, handle, name);
    }

    private native UserProfile usersModify(long qaul, String handle, String name);


    /**
     * Login as an existing user via their ID and password
     *
     * @param id the user ID
     * @param pw the user password
     * @return indicate whether the
     */
    public boolean usersLogin(Id id, String pw) {
        return usersLogin(libqaulState, id, pw);
    }

    private native boolean usersLogin(long qaul, Id id, String pw);

    /**
     * List available chat rooms for the current session
     *
     * @return a list of available chat rooms
     */
    public ArrayList<ChatRoom> chatList() {
        return chatList(libqaulState);
    }

    private native ArrayList<ChatRoom> chatList(long qaul);

    /**
     * Start a new chat with some friends.
     *
     * @param name    the name of the chat room.  When none is given, in a 1-on-1
     *                the name of the friend will be used, and for a group chat a
     *                random name will be generated
     * @param friends a set of remote users on the network to talk to
     * @return the room ID for further commands
     */
    public ChatRoom chatStart(String name, ArrayList<Id> friends) {
        return chatStart(libqaulState, name, friends);
    }

    private native ChatRoom chatStart(long qaul, String name, ArrayList<Id> friends);

    /**
     * Get a room object for a particular Id
     *
     * @param id the room identifier
     * @return The room associated with the given id
     */
    public ChatRoom chatGetRoom(Id id) {
        return chatGetRoom(libqaulState, id);
    }

    private native ChatRoom chatGetRoom(long qaul, Id id);

    /**
     * Send a text message to a room
     *
     * @param room    the room ID
     * @param content the chat message content
     * @return the created chat message to display
     */
    public ChatMessage chatSendMessage(Id room, String content) {
        return chatSendMessage(libqaulState, room, content);
    }

    private native ChatMessage chatSendMessage(long qaul, Id room, String content);

    /**
     * Load all messages from a chat room
     *
     * @param room the room ID to load
     * @return a list of messages in this room
     */
    public ArrayList<ChatMessage> chatLoadMessages(Id room) {
        return chatLoadMessages(libqaulState, room);
    }

    private native ArrayList<ChatMessage> chatLoadMessages(long qaul, Id room);

    /**
     * Receive a data frame via wifi direct
     * <p>
     * The ID is the sender identity
     *
     * @param encodedFrame encoded data frame, ignored by Java code and passed into Rust
     */
    public void wdReceived(byte[] encodedFrame) {
        wdReceived(this.libqaulState, encodedFrame);
    }

    private native void wdReceived(long qaul, byte[] encodedFrame);

    /**
     * Get a frame from the Rust code to send off to someone special
     *
     * @return the next frame to send off with target informatieon
     */
    public Frame wdToSend() {
        return wdToSend(this.libqaulState);
    }

    private native Frame wdToSend(long qaul);
}
