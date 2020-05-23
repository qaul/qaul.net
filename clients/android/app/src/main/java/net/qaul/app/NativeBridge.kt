package net.qaul.app

/** A bridge interface for native initialisation */
class NativeBridge {
    private val libqaulState: Long? = null;

    protected fun init() {

    }

    /**
     * Debugging method only, probably should be removed.
     *
     * @param to input string to the Rust code
     * @return different string based on the input
     */
    external fun hello(to: String?): String?

    /**
     * Start the main application server.
     *
     * This will bootstrap the libqaul service stack from the bottom up,
     * starting with the router and network modules.  Make sure that
     * #{wdSetup} and #{wdSendHook} are available to the native run context.
     *
     * @param port the port to run the webgui http server on
     * @param path the path to the webgui sources in internal storage
     *
     * @return application pointer to the libqaul android state
     *         for future transactions (currently not used)
     */
    protected external fun startServer(port: Int, path: String?): Long;
}