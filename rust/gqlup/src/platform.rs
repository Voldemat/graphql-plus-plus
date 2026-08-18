pub fn asset_name() -> Result<&'static str, String> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => Ok("gql-linux-x86_64"),
        ("linux", "aarch64") => Ok("gql-linux-arm64"),
        ("macos", "x86_64") => Ok("gql-darwin-x86_64"),
        ("macos", "aarch64") => Ok("gql-darwin-arm64"),
        (os, arch) => Err(format!(
            "Unsupported platform: {os}-{arch}. gqlup only supports \
             linux/darwin on x86_64/arm64."
        )),
    }
}
