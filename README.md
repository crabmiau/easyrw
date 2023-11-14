
# EasyRW

Easy reading and writing memory on rust.

**IM RATE LIMITED ON CRATES, USE THIS VERSION INSTEAD OF THE CRATES ONE FOR NOW**




## Assault Cube R/W Example:

```rust
use easyrw::memory::init;

fn main() {
    let proc = init("ac_client.exe", false).expect("Failed to attach to process"); //attach to process with false argument, means it will write memory externally, not internally, if ur making a dll then put it to true r/w internally
    let assault_cube = proc.get_assault_cube(proc.getbase("ac_client.exe")); // get assaultcube offsets, there is only 3 of them in this library just for example

    println!("HP: {}", proc.read::<i32>(assault_cube.hp)); // print hp
    println!("Nades: {}", proc.read::<i32>(assault_cube.nades)); // print grenades
    println!("Armor: {}", proc.read::<i32>(assault_cube.armor)); // print armor
}
```

## Overall R/W Example:
```rust
use easyrw::memory::init;

fn main()  {
    let proc = init("ac_client.exe", false).expect("Failed to attach to process"); //attach to process with false argument, means it will write memory externally, not internally, if ur making a dll then put it to true r/w internally
    let base = proc.getbase("ac_client.exe"); // get module base
    proc.write(proc.get_ptr(base + 0x17E0A8, &[0xEC]), 104); // assault cube example: write 104 to hp address
}
```

