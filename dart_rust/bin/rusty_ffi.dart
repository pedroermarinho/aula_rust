import 'dart:ffi' as ffi;
import 'dart:ffi';
import 'package:ffi/ffi.dart';
typedef NativeRustPlayOnceFunction = ffi.Void Function(ffi.Pointer<Utf8>);
typedef NativeRustPlayOnceFunction2 = void Function(ffi.Pointer<Utf8>);
typedef NativePlayOnceFunction = void Function();
void main() {
 
  var dl =
      ffi.DynamicLibrary.open('target/debug/libplay_once.so');
 
  var play_once =
      dl.lookupFunction<NativeRustPlayOnceFunction,NativeRustPlayOnceFunction2>(
          'play_once');
 
  final song = Utf8.toUtf8('beep.wav').cast();
  play_once(song);
}