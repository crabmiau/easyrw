
# EasyRW

Easy reading and writing memory on rust.




## Installation

Run the following command in your project directory:
```bash
cargo add easyrw
```

**OR**

add the following line to your Cargo.toml:
```
easyrw = "0.2.2"
```
    
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

## a BAD Read Range Example:
```rust
use easyrw::memory::init;

fn main()  {
    let proc = init("ac_client.exe", false).expect("Failed to attach to process"); //attach to process with false argument, means it will write memory externally, not internally, if ur making a dll then put it to true r/w internally
    let base = proc.getbase("ac_client.exe"); // get module base
    if let Some(data) = proc.read_range::<i32>(
        proc.get_ptr(base + 0x17E0A8, &[0x108]),
        proc.get_ptr(base + 0x17E0A8, &[0x140]),
    ) {
        println!("data: {:?}", data);
    } else {
        println!("failed to read memory range");
    }
}
```


## a BAD Write Range Example:
```rust
use easyrw::memory::init;

fn main()  {
    let proc = init("ac_client.exe", false).expect("Failed to attach to process"); //attach to process with false argument, means it will write memory externally, not internally, if ur making a dll then put it to true r/w internally
    let base = proc.getbase("ac_client.exe"); // get module base
    proc.write_range(proc.get_ptr(base + 0x17E0A8, &[0x108]), &[999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999, 999]); // this is cursed but it will write from base + 0x17E0A8, &[0x108] to base + 0x17E0A8, &[0x140]
}
```
