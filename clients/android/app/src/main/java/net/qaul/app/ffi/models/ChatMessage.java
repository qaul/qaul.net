package net.qaul.app.ffi.models;

/**
 * A chat message that is part of a chat room.
 *
 * The actual message-room association isn't made here because it's
 * irrelevant for this client (for now - see notifications)
 */
public class ChatMessage {
    public Id id;
    public Id sender;
    public String timestamp;
    public String content;

    public ChatMessage(Id id, Id sender, String timestamp, String content) {
        this.id = id;
        this.sender = sender;
        this.timestamp = timestamp;
        this.content = content;
    }
}
