package net.qaul.app.ffi.models

data class NetworkFile(val name: String, val size: Int, val type: FileType) {
    // This is kind of a stupid way of dealing with file types
    enum class FileType(extension: String.Companion) {
        TEXT(String),
        PICTURE(String),
        VIDEO(String),
        AUDIO(String),
        RAW(String)
    }
}