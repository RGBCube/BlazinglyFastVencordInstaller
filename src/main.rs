mod discord;

fn main() -> anyhow::Result<()> {
    let found_installations = discord::find()?;

    println!("{:?}", found_installations);

    Ok(())
}
