use clap::Parser;
use lalrpop_util::lalrpop_mod;
use std::fs::{read_to_string, File, self};
use std::io::Write;

lalrpop_mod!(asm);


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Raw assembly file
    input: String,

    // Output Path
    #[clap(short, long, default_value = "a.out")]
    out: String,

    // Data segment raw file
    #[clap(short, long)]
    data_file: String,
}

fn main() {
    let args = Args::parse();

    println!("Input file: {}", &args.input);
    println!("Data segment raw file: {}", &args.data_file);

    let input = read_to_string(args.input).unwrap();

    let data_segment: Vec<u8> = fs::read(args.data_file).unwrap();

    let ast = asm::OperationsParser::new().parse(&input).unwrap();

    println!("{:?}", ast);

    let mut file = File::create(&args.out).unwrap();

    let header: &[u8] = &[0x1e, 0x55, 0xc6, 0xb3, // Magic Number
                    0x00, 0x00, // Version
                    0x00 << 4 | 0x00, // Enable Segment Compress (Code + Other)
                    0x00 // Compress Option (1 means lz)
    ];

    let mut seg_table: [u8; 31] = [0x02, // Segment count
            0x01, // Execuable
            0x5, b'.', b'c', b'o', b'd', b'e', // segment length and name
            39, 0x0, 0x0, 0x0, // Offest(header.len + seg_table.len)
            0x0, 0x0, 0x0, 0x0, // Length(unknown)
            0x00, // Unexecuable
            0x5, b'.', b'd', b'a', b't', b'a', // segment length and name
            0x0, 0x0, 0x0, 0x0, // Offest(unknown)
            0x0, 0x0, 0x0, 0x0, // Length(unknown)
    ];

    file.write(header).unwrap();

    let mut generated_code_segment: Vec<u8> = vec![];

    for operation in ast {
        generated_code_segment.extend(operation.generate());
    }

    let len: u32 = generated_code_segment.len() as u32;
    // code length
    seg_table[12] = len as u8;
    seg_table[13] = (len >> 8) as u8;
    seg_table[14] = (len >> 16) as u8;
    seg_table[15] = (len >> 24) as u8;


    let len: u32 = 39 + len;
    // data offest
    seg_table[23] = len as u8;
    seg_table[24] = (len >> 8) as u8;
    seg_table[25] = (len >> 16) as u8;
    seg_table[26] = (len >> 24) as u8;


    let data_len: u32 = data_segment.len() as u32;
    // data length
    seg_table[27] = data_len as u8;
    seg_table[28] = (data_len >> 8) as u8;
    seg_table[29] = (data_len >> 16) as u8;
    seg_table[30] = (data_len >> 24) as u8;

    file.write(&seg_table).unwrap();
    file.write(&generated_code_segment).unwrap();
    file.write(&data_segment).unwrap();

    println!("Outputed in {}", &args.out);
}
