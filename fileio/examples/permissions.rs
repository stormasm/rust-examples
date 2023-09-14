use std::fs::File;
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;

fn main() -> std::io::Result<()> {
    let f = File::create("foo.txt")?;
    let metadata = f.metadata()?;
    let _permissions = metadata.permissions();

    let mut permissions = Permissions::from_mode(0o755);

    permissions.set_mode(0o755); // Read/write/execute for owner and read for others.
    assert_eq!(permissions.mode(), 0o755);
    Ok(())
}
