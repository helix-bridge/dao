use ethers::contract::MultiAbigen;

fn main() {
    let dir = "./abi/";
    let binding = "./cli/src/bindings/";
    let gen = MultiAbigen::from_json_files(dir).unwrap();
    gen.write_to_module(binding).unwrap();
}
