use std::io::Write;

fn next_perm<const OUTPUT_COUNT: usize>(perm: &mut [usize; OUTPUT_COUNT], max: usize) {
    for f in perm.iter_mut().rev() {
        *f += 1;

        if *f >= max {
            *f = 0;
        } else {
            break;
        }
    }
}

fn write_field_list<const OUTPUT_COUNT: usize>(
    stream: &mut impl Write,
    perm: &[usize; OUTPUT_COUNT],
) -> std::io::Result<()> {
    for i in 0..OUTPUT_COUNT {
        if i > 0 {
            write!(stream, ", ")?;
        }

        write!(stream, "{}", perm[i])?;
    }

    for _ in OUTPUT_COUNT..OUTPUT_COUNT.next_power_of_two() {
        write!(stream, ", 0")?;
    }

    Ok(())
}

fn write_swizzles<const COMPONENT_COUNT: usize, const OUTPUT_COUNT: usize>(
    stream: &mut impl Write,
    element_type: &str,
    support_alt_fields: bool,
) -> std::io::Result<()> {
    const FIELD_NAMES: [&str; 4] = ["x", "y", "z", "w"];
    #[cfg(feature = "color_fields")]
    const ALT_FIELD_NAMES: [&str; 4] = ["r", "g", "b", "a"];
    let result_ty = format!("Vector{}{}", OUTPUT_COUNT, element_type);

    let mut perm = [0; OUTPUT_COUNT];
    let perm_count = COMPONENT_COUNT.pow(OUTPUT_COUNT as u32);

    for i in 0..perm_count {
        if i > 0 {
            writeln!(stream)?;
        }

        writeln!(stream, "    #[allow(missing_docs)]")?;
        writeln!(stream, "    #[inline]")?;
        write!(stream, "    pub fn ")?;
        for f in perm.map(|f| FIELD_NAMES[f]) {
            write!(stream, "{f}")?;
        }
        writeln!(stream, "(&self) -> {result_ty} {{")?;
        write!(
            stream,
            "        {result_ty}::from_simd_truncate(simd_swizzle!(self.0, ["
        )?;
        write_field_list(stream, &perm)?;
        writeln!(stream, "]))")?;
        writeln!(stream, "    }}")?;

        #[cfg(feature = "color_fields")]
        if support_alt_fields {
            writeln!(stream)?;

            writeln!(stream, "    #[allow(missing_docs)]")?;
            writeln!(stream, "    #[inline]")?;
            write!(stream, "    pub fn ")?;
            for f in perm.map(|f| ALT_FIELD_NAMES[f]) {
                write!(stream, "{f}")?;
            }
            writeln!(stream, "(&self) -> {result_ty} {{")?;
            write!(
                stream,
                "        {result_ty}::from_simd_truncate(simd_swizzle!(self.0, ["
            )?;
            write_field_list(stream, &perm)?;
            writeln!(stream, "]))")?;
            writeln!(stream, "    }}")?;
        }

        next_perm(&mut perm, COMPONENT_COUNT);
    }

    Ok(())
}

fn main() {
    use std::env;
    use std::fs;
    use std::io::BufWriter;
    use std::path::Path;

    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir: &Path = out_dir.as_ref();
    fs::create_dir_all(out_dir).unwrap();

    let out_file = out_dir.join("swizzle.rs");
    let mut out_file = BufWriter::new(fs::File::create(out_file).unwrap());

    writeln!(out_file, "impl Vector2f {{").unwrap();
    write_swizzles::<2, 2>(&mut out_file, "f", true).unwrap();
    writeln!(out_file).unwrap();
    write_swizzles::<2, 3>(&mut out_file, "f", true).unwrap();
    writeln!(out_file).unwrap();
    write_swizzles::<2, 4>(&mut out_file, "f", true).unwrap();
    writeln!(out_file, "}}").unwrap();

    writeln!(out_file, "impl Vector3f {{").unwrap();
    write_swizzles::<3, 2>(&mut out_file, "f", true).unwrap();
    writeln!(out_file).unwrap();
    write_swizzles::<3, 3>(&mut out_file, "f", true).unwrap();
    writeln!(out_file).unwrap();
    write_swizzles::<3, 4>(&mut out_file, "f", true).unwrap();
    writeln!(out_file, "}}").unwrap();

    writeln!(out_file, "impl Vector4f {{").unwrap();
    write_swizzles::<4, 2>(&mut out_file, "f", true).unwrap();
    writeln!(out_file).unwrap();
    write_swizzles::<4, 3>(&mut out_file, "f", true).unwrap();
    writeln!(out_file).unwrap();
    write_swizzles::<4, 4>(&mut out_file, "f", true).unwrap();
    writeln!(out_file, "}}").unwrap();

    writeln!(out_file, "impl Vector2i {{").unwrap();
    write_swizzles::<2, 2>(&mut out_file, "i", false).unwrap();
    writeln!(out_file).unwrap();
    write_swizzles::<2, 3>(&mut out_file, "i", false).unwrap();
    writeln!(out_file).unwrap();
    write_swizzles::<2, 4>(&mut out_file, "i", false).unwrap();
    writeln!(out_file, "}}").unwrap();

    writeln!(out_file, "impl Vector3i {{").unwrap();
    write_swizzles::<3, 2>(&mut out_file, "i", false).unwrap();
    writeln!(out_file).unwrap();
    write_swizzles::<3, 3>(&mut out_file, "i", false).unwrap();
    writeln!(out_file).unwrap();
    write_swizzles::<3, 4>(&mut out_file, "i", false).unwrap();
    writeln!(out_file, "}}").unwrap();

    writeln!(out_file, "impl Vector4i {{").unwrap();
    write_swizzles::<4, 2>(&mut out_file, "i", false).unwrap();
    writeln!(out_file).unwrap();
    write_swizzles::<4, 3>(&mut out_file, "i", false).unwrap();
    writeln!(out_file).unwrap();
    write_swizzles::<4, 4>(&mut out_file, "i", false).unwrap();
    writeln!(out_file, "}}").unwrap();

    writeln!(out_file, "impl Quaternion {{").unwrap();
    write_swizzles::<4, 2>(&mut out_file, "f", false).unwrap();
    writeln!(out_file).unwrap();
    write_swizzles::<4, 3>(&mut out_file, "f", false).unwrap();
    writeln!(out_file).unwrap();
    write_swizzles::<4, 4>(&mut out_file, "f", false).unwrap();
    writeln!(out_file, "}}").unwrap();
}
