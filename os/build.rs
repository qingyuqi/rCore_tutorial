use std::fs::File;
use std::io::{Result, Write};

fn main() {
    println!("cargo:rerun-if-env-changed=USER_IMG");
    if let Ok(user_img) = std::env::var("USER_IMG") {
        println!("cargo:rerun-if-changed={}", user_img);
    }
    gen_link_user_asm().unwrap();
}

/// Generate assembly file for linking user image
fn gen_link_user_asm() -> Result<()> {
    let mut f = File::create("src/link_user.S").unwrap();
    let user_img = std::env::var("USER_IMG").unwrap();

    writeln!(f, "# generated by build.rs - do not edit")?;
    writeln!(
        f,
        r#"
    .section .data
    .global _user_img_start
    .global _user_img_end
_user_img_start:
    .incbin "{}"
_user_img_end:
"#,
        user_img
    )?;
    Ok(())
}
