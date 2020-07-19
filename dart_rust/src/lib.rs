use rodio::source::Source;
use rodio;
use std::fs::File;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn play_once(ptr: *const c_char){
    let cstr = unsafe { CStr::from_ptr(ptr) }; //just two lines changed
    let device = rodio::default_output_device().unwrap(); // instantiate rodio with the default speaker
    
    let file = File::open(cstr.to_str().unwrap()).unwrap(); // open file named beep.wav
    
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    
    rodio::play_raw(&device,source.convert_samples()); // play audio
    sleep(Duration::from_millis(1000)); // wait 1.5 s until stop playing
}
