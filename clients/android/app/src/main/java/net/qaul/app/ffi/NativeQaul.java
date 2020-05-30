package net.qaul.app.ffi;

import net.qaul.app.ffi.models.ChatMessage;
import net.qaul.app.ffi.models.ChatRoom;

import java.util.ArrayList;

/**
 * The native libqaul bridge interface.
 *
 * This file/class is written in Java because FFI integration between Kotlin and Rust
 * might be more complicated than with Java (for example javah exists, where there
 * doesn't seem to be a comparable kotlinh).  This can be changed in the future, and
 * this should definitely remain the only Java code, but this is simpler for now.
 */
public class NativeQaul {
    private Long QaulState = null;

    public NativeQaul(int port, String path) {
        this.startServer(port, path);

        this.checkLogin();
        this.chatLoadMessages("");
        this.chatSendMessage("", "");
        this.chatStart("", null);
        this.userRegister("", "");
        this.chatList();
    }

    /**
     * Start the main application server.
     *
     * This will bootstrap the libqaul service stack from the bottom up,
     * starting with the router and network modules.  Make sure that
     * #{wdSetup} and #{wdSendHook} are available to the native run context.
     *
     * @param port the port to run the webgui http server on
     * @param path the path to the webgui sources in internal storage
     */
    private native void startServer(int port, String path);

    /**
     * Check if the instance has a valid login
     *
     * @return true if login is valid
     */
    private native boolean checkLogin();

    /**
     * Register a new user on the local instance
     *
     * This also logs-in the user and starts advertising the ID across
     * the network.  This will allow others on the network to route packets
     * to this user.
     *
     * @param name optional name on the network advertised to others
     * @param password used to protect local files and assets
     */
    public native void userRegister(String name, String password);

    /**
     * List available chat rooms for the current session
     *
     * @return a list of available chat rooms
     */
    public native ArrayList<ChatRoom> chatList();

    /**
     * Start a new chat with some friends.
     *
     * @param name the name of the chat room.  When none is given, in a 1-on-1
     *             the name of the friend will be used, and for a group chat a
     *             random name will be generated
     * @param friends a set of remote users on the network to talk to
     * @return the room ID for further commands
     */
    public native String chatStart(String name, ArrayList<String> friends);

    /**
     * Send a text message to a room
     *
     * @param room the room ID
     * @param content the chat message content
     * @return the created chat message to display
     */
    public native ChatMessage chatSendMessage(String room, String content);

    /**
     * Load all messages from a chat room
     *
     * @param room the room ID to load
     * @return a list of messages in this room
     */
    public native ArrayList<ChatMessage> chatLoadMessages(String room);
}
