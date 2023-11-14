use easyrw::memory::init;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assault_cube_offsets() {
        let proc = init("ac_client.exe", false).expect("Failed to attach to process");
        let assault_cube = proc.get_assault_cube(proc.getbase("ac_client.exe"));

        assert!(assault_cube.hp != 0);
        assert!(assault_cube.nades != 0);
        assert!(assault_cube.armor != 0);
    }

    #[test]
    fn test_read_write_memory() {
        let proc = init("ac_client.exe", false).expect("Failed to attach to process");
        let assault_cube = proc.get_assault_cube(proc.getbase("ac_client.exe"));

        let address_to_test = assault_cube.hp;
        let value_to_write = 42;

        assert!(proc.write(address_to_test, value_to_write));

        let read_value: i32 = proc.read(address_to_test);
        assert_eq!(read_value, value_to_write);
    }
}
