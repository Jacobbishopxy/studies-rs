use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Read},
    str::FromStr,
};

use memmap::Mmap;

// 1. 加载整个文件为一个字符串
pub fn read_all<T: FromStr>(file_name: &str) -> Vec<Result<T, <T as FromStr>::Err>> {
    fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse())
        .collect()
}

// 2. 使用 `lines()` 遍历器
pub fn read_iter(file_name: &str, func: fn(&str)) {
    let file = File::open(file_name).expect("file not found!");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        func(&line.unwrap());
    }
}

// 3. 使用 `read_line()` 函数
pub fn read_line(file_name: &str, func: fn(&str)) -> Result<(), std::io::Error> {
    let file = File::open(&file_name)?;

    let mut reader = BufReader::new(file);
    let mut line = String::new();

    loop {
        match reader.read_line(&mut line) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }

                func(&line);

                line.clear();
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    Ok(())
}

// 4. 使用 `read_split()` 遍历器
pub fn read_spilt(file_name: &str, func: fn(&[u8])) -> Result<(), std::io::Error> {
    let file = File::open(&file_name)?;

    let reader = BufReader::new(file);

    for line in reader.split(0x10) {
        func(&line?);
    }

    Ok(())
}

// 5. 使用 `mmap()` api
pub fn read_mmap(file_name: &str) -> Result<(), std::io::Error> {
    let file = File::open(&file_name)?;

    let mmap = unsafe { Mmap::map(&file)? };

    for s in mmap.split(|x| *x == 0x10) {
        println!("{:?}", std::str::from_utf8(&s).unwrap());
    }

    Ok(())
}

// 6. （特殊）读取二进制文件
pub fn read_png(file_name: &str) -> Result<(), std::io::Error> {
    const BUFFER_SIZE: usize = 256;

    let mut file = File::open(&file_name)?;

    let mut buffer = [0; BUFFER_SIZE];

    let _ = file.by_ref().take(8).read(&mut buffer)?;
    assert_eq!(&buffer[1..4], "PNG".as_bytes());

    let chunk_size = file.read_u32::<BigEndian>().unwrap();
    let _ = file.by_ref().take(4).read(&mut buffer)?;
    assert_eq!(&buffer[0..4], "IHDR".as_bytes());

    let image_width = file.read_u32::<BigEndian>().unwrap();
    let image_height = file.read_u32::<BigEndian>().unwrap();
    println!("image is W={} x H={}", image_width, image_height);

    Ok(())
}

fn main() {
    println!("test");
}
