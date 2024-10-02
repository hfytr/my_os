use std::env;
use std::fs::read;

pub const UEFI_PATH: &str = env!("UEFI_PATH");
pub const BIOS_PATH: &str = env!("BIOS_PATH");
pub const KERNEL_BINARY: &str = env!("KERNEL_BINARY");

fn _read_psf1(path: &str) -> String {
    let bytes = read(path).unwrap();

    let flags_byte = bytes[2];
    let unicode_table = flags_byte & 0x11 > 0;
    // 256 if bit not set, else 512
    let num_glyphs = (flags_byte & 0x1) as u16 * 256 + 256;
    let glyph_size = bytes[3];

    println!(
        "unicode_table: {}, num_glyphs: {}, glyph size: {}",
        unicode_table, num_glyphs, glyph_size
    );

    let mut glyphs = Vec::new();
    let glyphs_iter = bytes[4..]
        .chunks(glyph_size as usize)
        .take(num_glyphs as usize);
    for glyph in glyphs_iter {
        glyphs.push(format!(
            "0x{}",
            glyph
                .iter()
                .rev()
                .map(|b| format!("{:02x}", b.reverse_bits()))
                .collect::<Vec<String>>()
                .join("")
        ));

        // print out the font in ascii
        for (i, byte) in glyph.iter().enumerate() {
            println!(
                "{}",
                String::from_iter(format!("{:08b}", byte).as_bytes().into_iter().map(|&c| {
                    if c == b'0' {
                        " "
                    } else {
                        "&"
                    }
                }))
            );
            if i % 16 == 15 {
                println!("------------------");
            }
        }
    }

    // ik its not alway u128
    format!(
        "pub const FONT: [u128; {}] = [{}];",
        num_glyphs,
        glyphs.join(",")
    )
}

fn main() {
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");

    let args = env::args().collect::<Vec<_>>();
    let mut qemu_cmd = std::process::Command::new("qemu-system-x86_64");
    if args.contains(&String::from("-u")) {
        println!("UEFI: {}", uefi_path);
        qemu_cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
        qemu_cmd
            .arg("-drive")
            .arg(format!("format=raw,file={uefi_path}"));
    } else if args.contains(&String::from("-b")) {
        println!("BIOS: {}", bios_path);
        qemu_cmd
            .arg("-drive")
            .arg(format!("format=raw,file={bios_path}"));
    } else {
        panic!("specify -b for bios or -u for uefi")
    };

    if args.contains(&String::from("-d")) {
        println!("generating debug.lldb");
        qemu_cmd.args(["-s", "-S"]);

        let lldb_content = format!(
            r#"target create {KERNEL_BINARY}
            target modules load --file {KERNEL_BINARY} --slide 0xffff800000000000
            gdb-remote localhost:1234"#
        );
        std::fs::write("debug.lldb", lldb_content).expect("unable to create lldb debug file");

        let mut lldb_cmd = std::process::Command::new("lldb");
        lldb_cmd.args(["-s", "debug.lldb"]);

        // make sure to spawn qemu first
        let mut qemu_process = qemu_cmd.spawn().unwrap();
        let mut lldb_process = lldb_cmd.spawn().unwrap();
        lldb_process.wait().unwrap();
        qemu_process.wait().unwrap();
    } else {
        let mut child = qemu_cmd.spawn().unwrap();
        child.wait().unwrap();
    }
}
