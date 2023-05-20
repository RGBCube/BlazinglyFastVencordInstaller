mod discord;

fn main() -> anyhow::Result<()> {
    let _found_installations = discord::find()?;

    for installation in _found_installations {
        println!("{:?}", installation);
    }

    Ok(())
}
