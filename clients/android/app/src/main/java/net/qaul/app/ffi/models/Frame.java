package net.qaul.app.ffi.models;

/** A mostly opaque mapping type to enable Java to receive two fields */
public class Frame {
    public int target;
    public byte[] data;

    public Frame(int target, byte[] data) {
        this.target = target;
        this.data = data;
    }
}
