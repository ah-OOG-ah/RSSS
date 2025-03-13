use std::fs::copy;
use spirv_builder::SpirvBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hey! I'm awake!");

    let result = SpirvBuilder::new("./rss", "spirv-unknown-vulkan1.1")
        .build()?;
    std::fs::create_dir_all("./spirv")?;
    copy(result.module.unwrap_single(), "./spirv/shader.spv")?;
    Ok(())
}