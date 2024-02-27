use std::{
    fs::read_to_string,
    hash::{DefaultHasher, Hash, Hasher},
    process::Command,
};

fn main() {
    // If any TS changes, re-run the build script
    println!("cargo:rerun-if-changed=src/*.ts");

    // for entry in ["common", "form", "interpreter"].iter() {
    //     gen_bindings(entry);
    // }
}

// okay...... so tsc might fail if the user doesn't have it installed
// we don't really want to fail if that's the case
// but if you started *editing* the .ts files, you're gonna have a bad time
// so.....
// we need to hash each of the .ts files and add that hash to the JS files
// if the hashes don't match, we need to fail the build
// that way we also don't need
fn gen_bindings(name: &str) {
    let contents = read_to_string(&format!("src/{name}.ts")).unwrap();
    let generated = read_to_string(&format!("src/gen/{name}.js")).unwrap_or_default();
    let hashed = hash_file(&contents);

    // If the file is generated, and the hash is the same, we're good, don't do anything
    if generated
        .lines()
        .next()
        .unwrap_or_default()
        .starts_with(&format!("// DO NOT EDIT THIS FILE. HASH: {}", hashed))
    {
        return;
    }

    // If the file is generated, and the hash is different, we need to generate it
    let status = Command::new("tsc")
        .arg(format!("src/{name}.ts"))
        .arg("--outDir")
        .arg("gen")
        .arg("--target")
        .arg("es6")
        .status()
        .unwrap();

    if !status.success() {
        panic!(
            "Failed to generate bindings for {}. Make sure you have tsc installed",
            name
        );
    }

    // The file should exist, and now we need write the TS hash to the file
    let generated = read_to_string(&format!("gen/{name}.js")).unwrap();
    let generated = format!("// DO NOT EDIT THIS FILE. HASH: {}\n{}", hashed, generated);
    std::fs::write(&format!("src/gen/{name}.js"), generated).unwrap();
}

fn hash_file(obj: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}
