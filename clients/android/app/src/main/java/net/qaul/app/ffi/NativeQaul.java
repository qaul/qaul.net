package net.qaul.app.ffi;

import net.qaul.app.ffi.models.ChatMessage;
import net.qaul.app.ffi.models.ChatRoom;
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
     * List available users
     *
     * @return List of local users
     */
    public ArrayList<UserProfile> usersList() {
        return usersList(libqaulState);
    }

    private native ArrayList<UserProfile> usersList(long qaul);

    /**
     * Login as an existing user via their ID and password
     *
     * @param id the user ID
     * @param pw the user password
     * @return indicate whether the
     */
    public boolean usersLogin(String id, String pw) { return usersLogin(libqaulState, id, pw); }

    private native boolean usersLogin(long qaul, String id, String pw);

    /**
     * Register a new user on the local instance
     * <p>
     * This also logs-in the user and starts advertising the ID across
     * the network.  This will allow others on the network to route packets
     * to this user.
     *
     * @param name     optional name on the network advertised to others
     * @param password used to protect local files and assets
     */
    public native void userRegister(long qaul, String name, String password);

    /**
     * List available chat rooms for the current session
     *
     * @return a list of available chat rooms
     */
    public native ArrayList<ChatRoom> chatList(long qaul);

    /**
     * Start a new chat with some friends.
     *
     * @param name    the name of the chat room.  When none is given, in a 1-on-1
     *                the name of the friend will be used, and for a group chat a
     *                random name will be generated
     * @param friends a set of remote users on the network to talk to
     * @return the room ID for further commands
     */
    public native String chatStart(long qaul, String name, ArrayList<String> friends);

    /**
     * Send a text message to a room
     *
     * @param room    the room ID
     * @param content the chat message content
     * @return the created chat message to display
     */
    public native ChatMessage chatSendMessage(long qaul, String room, String content);

    /**
     * Load all messages from a chat room
     *
     * @param room the room ID to load
     * @return a list of messages in this room
     */
    public native ArrayList<ChatMessage> chatLoadMessages(long qaul, String room);
}
