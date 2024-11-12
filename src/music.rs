// Implementation of cpal intro + https://github.com/crumblingstatue/rust-openmpt-sys/blob/master/examples/play.rs
// (that is to say, i copied the openmpt code since i could find like zero documentation and didn't
// care uhh)

use std::ffi::c_void;

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Stream,
};
use cpal::{SampleFormat, SampleRate, SupportedStreamConfigRange};

// TODO: get rid of this? We want to work with hardware that doesn't support it
fn desired_config(cfg: &SupportedStreamConfigRange) -> bool {
    cfg.channels() == 2
        && cfg.sample_format() == SampleFormat::F32
        && cfg.max_sample_rate() >= SampleRate(48_000)
}

#[derive(Debug, Clone)]
struct Module {
    handle: *mut openmpt_sys::openmpt_module,
}

impl Module {
    fn read(&mut self, rate: i32, data: &mut [f32]) {
        unsafe {
            // Infinite repeat
            openmpt_sys::openmpt_module_set_repeat_count(self.handle, -1);
            openmpt_sys::openmpt_module_read_interleaved_float_stereo(
                self.handle,
                rate,
                data.len() / 2,
                data.as_mut_ptr(),
            );
        };
    }
}

unsafe impl Send for Module {}

pub struct ModulePlayer {
    module: Module,
    stream: Option<Stream>,
}

impl ModulePlayer {
    pub fn new(file_path: &str) -> Self {
        // Set up module data
        let mod_data = std::fs::read(file_path).unwrap();
        let mod_handle = unsafe {
            openmpt_sys::openmpt_module_create_from_memory2(
                mod_data.as_ptr() as *const c_void,
                mod_data.len(),
                None,
                std::ptr::null_mut(),
                None,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null(),
            )
        };

        // Create ModulePlayer
        ModulePlayer {
            module: Module { handle: mod_handle },
            stream: None,
        }
    }

    pub fn generate_stream(&mut self) {
        // Set up cpal, build stream
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("ModulePlayer: No output device available");

        let mut supported_cfgs = device.supported_output_configs().unwrap();
        let Some(config) = supported_cfgs.find(desired_config) else {
            panic!("ModulePlayer: Output device doesn't support desired parameters");
        };
        let config = config.with_sample_rate(SampleRate(48_000)).config();

        let mut module = self.module.clone();

        self.stream = Some(
            device
                .build_output_stream(
                    &config,
                    move |data: &mut [f32], _cpal| module.read(config.sample_rate.0 as _, data),
                    |err| {
                        dbg!(err);
                    },
                    None, // None=blocking, Some(Duration)=timeout
                )
                .unwrap(),
        );
    }

    pub fn play(&mut self) {
        if let Some(stream) = &mut self.stream {
            stream.play().unwrap();
        }
    }
}
