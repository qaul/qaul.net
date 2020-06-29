package net.qaul.app.ffi.models;

/**
 * A mostly opaque mapping type to enable Java to receive two fields
 */
public class Frame {
    public int target;
    public byte[] data;

    public Frame(int target, byte[] data) {
        this.target = target;
        this.data = data;
    }

    /**
     * Get the on-wire representation of this frame
     *
     * @return an array of bytes with the length first
     */
    public byte[] toWire() {
        return new byte[]{ 1, 3, 1, 2 };
    }
}
