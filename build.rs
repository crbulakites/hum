#[cfg(windows)]
extern crate vcpkg;

fn main() {
    #[cfg(windows)]
    {
        std::env::set_var("VCPKGRS_DYNAMIC" , "1");
        vcpkg::find_package("portaudio").unwrap();
    }
}
