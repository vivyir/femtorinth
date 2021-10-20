fn main() -> Result<(), Box<dyn std::error::Error>> {
    let results = femtorinth::search_mods("Sodium".into(), None, Some(1))?;
    let modres = results.hits[0].clone();
    println!("Found: {} from {}", &modres.title, &modres.author);
    println!("Description: {}", &modres.description);
    println!(
        "Downloads and follows: {} and {}",
        &modres.downloads, &modres.follows
    );
    println!(
        "Latest supported minecraft version: {}",
        &modres.latest_version
    );
    println!("Licensed under: {}", &modres.license);

    let version_list = femtorinth::version_list(modres.get_clean_id())?;
    let mod_version_data = version_list[0].clone();

    println!("Name of the newest version: {}", &mod_version_data.name);
    println!("Files for download with filenames:");
    for i in mod_version_data.files {
        println!("{}: {}", i.filename, i.url);
    }

    Ok(())
}
