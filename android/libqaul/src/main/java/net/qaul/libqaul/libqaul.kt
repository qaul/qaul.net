package net.qaul.libqaul

external fun dummy(): String
external fun start()
//external fun send()
//external fun receive()

fun loadLibqaul() {
    System.loadLibrary("libqaul")
}
