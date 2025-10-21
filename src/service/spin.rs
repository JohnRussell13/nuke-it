pub fn run(package_parts: Vec<&str>) -> Result<String, String> {
    // No need to handle
    if package_parts.len() != 2 {
        return Err(format!("Spin needs 1 argument: {package_parts:?}"));
    }

    let id = package_parts[1];
    println!("{id}");

    Ok(String::from("SPIN"))
}
