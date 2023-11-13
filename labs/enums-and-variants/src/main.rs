#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

#[derive(Debug)]
enum SizeUnit {
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
}

impl SizeUnit {
    fn from_str(unit: &str) -> Self {
        match unit {
            "bytes" => Self::Bytes,
            "kilobytes" => Self::Kilobytes,
            "megabytes" => Self::Megabytes,
            "gigabytes" => Self::Gigabytes,
            _ => panic!("Unknown unit"),
        }
    }
}

fn parse_size(size_str: &str) -> Sizes {
    let parts: Vec<&str> = size_str.split_whitespace().collect();
    let size = parts[0].parse::<u64>().unwrap();
    let unit = SizeUnit::from_str(parts[1]);

    let bytes = match unit {
        SizeUnit::Bytes => size,
        SizeUnit::Kilobytes => size * 1000,
        SizeUnit::Megabytes => size * 1_000_000,
        SizeUnit::Gigabytes => size * 1_000_000_000,
    };

    Sizes {
        bytes: format!("{} bytes", bytes),
        kilobytes: format!("{} kilobytes", bytes / 1000),
        megabytes: format!("{} megabytes", bytes / 1_000_000),
        gigabytes: format!("{} gigabytes", bytes / 1_000_000_000),
    }
}

fn main() {
    let sizes = parse_size("24 megabytes");
    println!("{:?}", sizes);
}
