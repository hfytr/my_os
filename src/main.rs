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
    let glyphs_iter = bytes.chunks(glyph_size as usize).take(num_glyphs as usize);
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
        cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
        cmd.arg("-drive")
            .arg(format!("format=raw,file={uefi_path}"));
    } else {
        cmd.arg("-drive")
            .arg(format!("format=raw,file={bios_path}"));
    }
    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}
