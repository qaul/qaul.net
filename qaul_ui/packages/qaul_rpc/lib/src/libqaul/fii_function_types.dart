part of 'libqaul.dart';

// referencing C API type definitions
// the type definitions to use

// start function
// C function definition:
//   void start();
typedef StartFunctionRust = Void Function();
typedef StartFunctionDart = void Function();

// start libqaul function on a desktop OS
// C function definition:
//   void start_desktop();
typedef StartDesktopFunctionRust = Void Function();
typedef StartDesktopFunctionDart = void Function();

// Check if libqaul finished initializing
// C function definition:
//   i32 initialization_finished();
typedef InitializationFinishedRust = Int32 Function();
typedef InitializationFinishedDart = int Function();

// hello function for testing
// Same signature for C and Dart.
// C function definition:
//   char *str hello();
typedef HelloFunctionRust = Pointer<Utf8> Function();
typedef HelloFunctionDart = Pointer<Utf8> Function();

// Get the number of rpc messages ever sent.
// C function definition:
//   i32 send_rpc_to_libqaul_count();
typedef SendRpcCounterRust = Int32 Function();
typedef SendRpcCounterDart = int Function();

// Get the number of rpc messages queued by libqaul to receive.
// C function definition:
//   i32 send_rpc_to_libqaul_count();
typedef ReceiveRpcQueuedRust = Int32 Function();
typedef ReceiveRpcQueuedDart = int Function();

// send protobuf RPC message to libqaul
// C function definition:
//   int32 send_rpc_to_libqaul( *uchar, uint32);
typedef SendRpcToLibqaulFunctionRust = Int32 Function(Pointer<Uint8>, Uint32);
typedef SendRpcToLibqaulFunctionDart = int Function(Pointer<Uint8>, int);

// check for protobuf RPC message from libqaul
// C function definition:
//   int32 receive_rpc_from_libqaul( *uchar, uint32);
typedef ReceiveRpcFromLibqaulFunctionRust = Int32 Function(Pointer<Uint8>, Uint32);
typedef ReceiveRpcFromLibqaulFunctionDart = int Function(Pointer<Uint8>, int);
