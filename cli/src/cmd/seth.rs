use cast::SimpleCast;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Encode calldata using abi.encode")]
pub enum Cast {
    Calldata {
        #[structopt(
            help = r#"When called with <sig> of the form <name>(<types>...), then perform ABI encoding to produce the hexadecimal calldata.
        If the value given—containing at least one slash character—then treat it as a file name to read, and proceed as if the contents were passed as hexadecimal data.
        Given data, ensure it is hexadecimal calldata starting with 0x and normalize it to lowercase.
        "#
        )]
        sig: String,
        args: Vec<String>,
    },
    #[structopt(name = "--calldata-decode")]
    #[structopt(
        about = "Decode ABI-encoded hex input data. Use `--abi-decode` to decode output data"
    )]
    CalldataDecode {
        #[structopt(
            help = "the function signature you want to decode, in the format `<name>(<in-types>)(<out-types>)`"
        )]
        sig: String,
        #[structopt(help = "the encoded calladata, in hex format")]
        calldata: String,
    },
    #[structopt(name = "--abi-decode")]
    #[structopt(
        about = "Decode ABI-encoded hex output data. Pass --input to decode as input, or use `--calldata-decode`"
    )]
    AbiDecode {
        #[structopt(
            help = "the function signature you want to decode, in the format `<name>(<in-types>)(<out-types>)`"
        )]
        sig: String,
        #[structopt(help = "the encoded calladata, in hex format")]
        calldata: String,
        #[structopt(long, short, help = "the encoded output, in hex format")]
        input: bool,
    },
    #[structopt(name = "abi-encode")]
    #[structopt(
        help = "ABI encodes the given arguments with the function signature, excluidng the selector"
    )]
    AbiEncode {
        #[structopt(help = "the function signature")]
        sig: String,
        #[structopt(help = "the list of function arguments")]
        args: Vec<String>,
    },
    #[structopt(name = "keccak")]
    #[structopt(about = "Keccak-256 hashes arbitrary data")]
    Keccak { data: String },
    #[structopt(name = "verify")]
    #[structopt(about = "verify two calldata is equel")]
    Verify { first: String, second: String },
}

impl Cast {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Cast::Calldata { sig, args } => {
                println!("{}", SimpleCast::calldata(sig, &args)?);
            }
            Cast::CalldataDecode { sig, calldata } => {
                let tokens = SimpleCast::abi_decode(&sig, &calldata, true)?;
                let tokens = foundry_utils::format_tokens(&tokens);
                tokens.for_each(|t| println!("{}", t));
            }
            Cast::AbiDecode {
                sig,
                calldata,
                input,
            } => {
                let tokens = SimpleCast::abi_decode(&sig, &calldata, input)?;
                let tokens = foundry_utils::format_tokens(&tokens);
                tokens.for_each(|t| println!("{}", t));
            }
            Cast::AbiEncode { sig, args } => {
                println!("{}", SimpleCast::abi_encode(&sig, &args)?);
            }
            Cast::Keccak { data } => {
                println!("{}", SimpleCast::keccak(&data)?);
            }
            Cast::Verify { first, second } => {
                let _first = SimpleCast::keccak(&first)?;
                let _second = SimpleCast::keccak(&second)?;
                println!("{:?}", _first.cmp(&_second));
            }
        }
        Ok(())
    }
}
