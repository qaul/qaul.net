package net.qaul.app.ffi;

/** Kind of a fake singleton? */
public class NativeState {
    private static NativeQaul qaul;

    public static void setup(NativeQaul q) {
        qaul = q;
    }

    public static NativeQaul get() {
        return qaul;
    }
}
