package net.qaul.libqaul

/// hello message from lib qaul
// dummy function for testing
external fun hello(): String

/// start libqaul
/// this also intializes the logging
external fun start()

/// check if libqaul has finished initializing
external fun initialized(): Boolean

/// get number of RPC messages sent to libqaul
/// this function is only for testing
external fun sendcounter(): Int

//external fun send()

/// how many RPC messages are queued by libqaul
/// to be received from this programme
external fun receivequeue(): Int

//external fun receive()

fun loadLibqaul() {
    System.loadLibrary("libqaul")
}
