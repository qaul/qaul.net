package net.qaul.app.ffi.models;

/**
 * A chat message that is part of a chat room.
 *
 * The actual message-room association isn't made here because it's
 * irrelevant for this client (for now - see notifications)
 */
public class ChatMessage {
    public Id id;
    public String timestamp;
    public String content;
    public Id author;

    public ChatMessage(Id id, String timestamp, String content, Id author) {
        this.id = id;
        this.timestamp = timestamp;
        this.content = content;
        this.author = author;
    }
}
