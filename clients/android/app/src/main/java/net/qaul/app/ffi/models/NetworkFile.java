package net.qaul.app.ffi.models;

/**
 * A file received or available on the network
 */
public class NetworkFile {
    public Id id;
    public String name;
    public int sizeInKb;
    public FileType type;
    public String extention;

    public enum FileType {
        Text,
        Picture,
        Video,
        Audio,
        Raw,
    }
}
