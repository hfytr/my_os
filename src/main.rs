use std::fs::read;

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

    let uefi = true;

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    if uefi {
        println!("{}", uefi_path);
        cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
        cmd.arg("-drive")
            .arg(format!("format=raw,file={uefi_path}"));
    } else {
        println!("{}", bios_path);
        cmd.arg("-drive")
            .arg(format!("format=raw,file={bios_path}"));
    }
    cmd.args(["-s", "-S"]);
    // 0x8000005ec0
    // add-symbol-file target/x86_64-unknown-none/debug/deps/artifact/kernel-d051de5109deb413/bin/kernel-d051de5109deb413 -o 0x8000005ec0

    const KERNEL_BINARY: &str = "target/x86_64-unknown-none/debug/deps/artifact/kernel-d051de5109deb413/bin/kernel-d051de5109deb413";
    let content = format!(
        r#"target create {KERNEL_BINARY}
target modules load --file {KERNEL_BINARY} --slide 0x8000005ec0
gdb-remote localhost:1234
b _start
c"#
    );
    std::fs::write("debug.lldb", content).expect("unable to create debug file");

    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}
