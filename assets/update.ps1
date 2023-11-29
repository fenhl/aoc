$last_dir = Get-Location
Get-ChildItem "crate" | Foreach-Object {
    cd $_.FullName
    cargo update
    cargo outdated -Rw
}
cd $last_dir
