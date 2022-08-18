use std::ffi::OsStr;
use std::fs::*;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::path::Path;

fn main() {
  let mut f = File::open("./data/HEROES2.AGG").unwrap();

  for file in parse_file_info(&f) {
    let fref = Read::by_ref(&mut f);
    let mut target = File::create("./extract/".to_string() + &file.name).unwrap();
    let mut buf: Vec<u8> = vec![0; file.size as usize];
    fref.seek(SeekFrom::Start(file.offset as u64)).unwrap();
    fref
      .take(file.size as u64)
      .read(buf.as_mut_slice())
      .unwrap();

    let ext = get_ext(&file.name).unwrap();

    let transformed = match ext {
      "BMP" => &buf[..],
      _ => &buf[..],
    };

    std::io::Write::write(&mut target, transformed).unwrap();
  }
}

fn le_to_u16(buf: &[u8; 2]) -> u16 {
  ((buf[1] as u16) << 8) | buf[0] as u16
}

fn le_to_u32(buf: &[u8]) -> u32 {
  ((buf[3] as u32) << 24) | ((buf[2] as u32) << 16) | ((buf[1] as u32) << 8) | buf[0] as u32
}

struct FileInfo {
  name: String,
  id: u32,
  size: u32,
  offset: u32,
}

fn parse_file_info(mut f: &File) -> Vec<FileInfo> {
  let mut n = 0;
  {
    let mut buf: [u8; 2] = [0; 2];
    let r = f.by_ref().take(2).read(&mut buf[..]);
    assert_eq!(r.unwrap(), 2);
    n = le_to_u16(&buf);

    println!("FileInfo len: {:?}", n);
  }

  let mut files = Vec::new();

  let mut offset = 0;
  let mut size = 0;

  for i in 0..n {
    let mut buf: [u8; 12] = [0; 12];
    let r = f.by_ref().take(12).read(&mut buf[..]);
    assert_eq!(r.unwrap(), 12);

    let id = le_to_u32(&buf[..4]);
    offset = le_to_u32(&buf[4..8]);
    size = le_to_u32(&buf[8..12]);

    files.push(FileInfo {
      name: "".to_string(),
      id,
      offset,
      size,
    });

    println!("[{}]: ID: {}; offset: {}; size: {}", i, id, offset, size)
  }

  let r = f
    .by_ref()
    .seek(SeekFrom::Start(offset as u64 + size as u64))
    .unwrap();
  assert_eq!(r, 43330111 + 10527);
  println!("Skipping to {}", r);

  for i in 0..n {
    let mut buf: [u8; 15] = [0; 15];
    let r = f.by_ref().take(15).read(&mut buf[..]);
    assert_eq!(r.unwrap(), 15);

    let mut char_vec = Vec::new();

    for x in 0..15 {
      if buf[x] == 0 {
        break;
      }
      char_vec.push(buf[x] as char);
    }

    let s = char_vec.iter().cloned().collect::<String>();

    println!("[{}]: filename: {}", i, s.trim());

    files[i as usize].name = s;
  }

  files
}

fn get_ext(filename: &str) -> Option<&str> {
  Path::new(filename).extension().and_then(OsStr::to_str)
}

fn convert_bmp(input: &[u8]) -> Vec<u8> {
  


  let mut output = Vec::new();

  output.push('B' as u8);
  output.push('M' as u8);

  // TODO: size
  for i in 0..4 {
    output.push(0);
  }

  for i in 0..4 {
    output.push(0);
  }

  // TODO: offset
  for i in 0..4 {
    output.push(0);
  }



  output
}
