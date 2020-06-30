package net.qaul.app.ffi.models;

import java.lang.reflect.Array;
import java.nio.ByteBuffer;

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
        ByteBuffer b = ByteBuffer.allocate(8);
        b.putInt(this.data.length);
        byte[] length = b.array();

        byte[] combine = new byte[length.length + data.length];
        int idx = 0;

        for (;idx < length.length; idx++) {
            combine[idx] = length[idx];
        }

        for (byte datum : data) {
            combine[idx] = datum;
            idx++;
        }

        return combine;
    }
}
