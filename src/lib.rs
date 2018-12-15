#![no_std]

#[macro_export]
macro_rules! entry {
    ($path: path) => {
        #[doc(hidden)]
        #[export_name = "main"]
        #[no_mangle]
        pub extern "C" fn __impl_main() -> ! {
            let f: fn() -> ! = $path;
            f()
        }
    };
}

#[macro_export]
macro_rules! irq_handler {
    ($path: path) => {
        #[doc(hidden)]
        #[export_name = "irq_handler"]
        #[no_mangle]
        pub extern "C" fn __impl_irq_handler() {
            let f: fn() -> () = $path;
            f()
        }
    };
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn reset_handler() -> ! {
    extern "C" {
        fn main() -> !; // we can pick any signature we want

        // Boundaries of the .bss section
        static mut _sbss: u32;
        static mut _ebss: u32;

        // Boundaries of the .data section
        static mut _sdata: u32;
        static mut _edata: u32;

        // Initial values of the .data section (stored in ROM)
        static _sidata: u32;
    }

    r0::zero_bss(&mut _sbss, &mut _ebss);
    r0::init_data(&mut _sdata, &mut _edata, &_sidata);

    main()
}
