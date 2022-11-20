package net.qaul.libqaul

import java.util.*



/// hello message from lib qaul
// dummy function for testing
external fun hello(): String
/// start libqaul
/// this also intializes the logging
external fun start(path: String)

/// check if libqaul has finished initializing
external fun initialized(): Boolean

/// get number of RPC messages sent to libqaul
/// this function is only for testing
external fun sendcounter(): Int

/// send an RPC message to libqaul
external fun send(message: ByteArray)

/// how many RPC messages are queued by libqaul
/// to be received from this programme
external fun receivequeue(): Int

/// receive an RPC message from libqaul
external fun receive(): ByteArray

/// send an SYS message from BLE library to libqaul
external fun syssend(message: ByteArray)

/// how many SYS messages are queued by libqaul
/// to be received from BLE module
external fun sysreceivequeue(): Int

/// receive an SYS message from libqaul to BLE library
external fun sysreceive(): ByteArray


/// load rust libqaul shared library
fun loadLibqaul() {
    System.loadLibrary("libqaul")
}
