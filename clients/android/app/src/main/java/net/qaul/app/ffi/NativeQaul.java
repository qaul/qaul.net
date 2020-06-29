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

    public NativeQaul(int port, String path) {
        this.libqaulState = startServer(port, path);
    }

    /**
     * Start the main application server.
     * <p>
     * This will bootstrap the libqaul service stack from the bottom up,
     * starting with the router and network modules.  Make sure that
     * #{wdSetup} and #{wdSendHook} are available to the native run context.
     *
     * @param port the port to run the webgui http server on
     * @param path the path to the webgui sources in internal storage
     */
    public native long startServer(int port, String path);

    /**
     * Check if the instance has a valid login
     *
     * @return true if login is valid
     */
    private native boolean checkLogin(long qaul);

    /**
     * Create a new user
     *
     */
    public void usersCreate(String handle, String name, String password) {
        usersCreate(libqaulState, handle, name, password);
    }

    private native void usersCreate(long qaul, String handle, String name, String password);

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
     * Get a particular user profile by ID
     *
     * @return List of local users
     */
    public UserProfile usersGet(Id id) {
        return usersGet(libqaulState, id);
    }

    private native UserProfile usersGet(long qaul, Id id);

    /**
     * Login as an existing user via their ID and password
     *
     * @param id the user ID
     * @param pw the user password
     * @return indicate whether the
     */
    public boolean usersLogin(String id, String pw) {
        return usersLogin(libqaulState, id, pw);
    }

    private native boolean usersLogin(long qaul, String id, String pw);

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
    public Id chatStart(String name, ArrayList<String> friends) {
        return chatStart(libqaulState, name, friends);
    }

    private native Id chatStart(long qaul, String name, ArrayList<String> friends);

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
     * @param target       interface specific mapping information if this endpoint is one-to-many
     * @param encodedFrame encoded data frame, ignored by Java code and passed into Rust
     */
    public void wdReceiveFrame(int target, byte[] encodedFrame) {
        wdReceiveFrame(this.libqaulState, target, encodedFrame);
    }

    private native void wdReceiveFrame(long qaul, int target, byte[] encodedFrame);

    /**
     * Get a frame from the Rust code to send off to someone special
     *
     * @return the next frame to send off with target informatieon
     */
    public Frame wdSendFrame() {
        return wdSendFrame(this.libqaulState);
    }

    private native Frame wdSendFrame(long qaul);
}
