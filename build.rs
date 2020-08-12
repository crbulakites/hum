extern crate peg;

#[cfg(windows)]
extern crate vcpkg;

fn main() {
    peg::cargo_build("src/hum_parse.rustpeg");

    #[cfg(windows)]
    {
        std::env::set_var("VCPKGRS_DYNAMIC", "1");
        vcpkg::find_package("portaudio").unwrap();
    }
}
