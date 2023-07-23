// The following is a hexdump of a zip64 file containing the following files:
// zero4400: 4400 MB of zeroes
// zero100: 100 MB of zeroes
// zero4400_2: 4400 MB of zeroes
//
// 00000000  50 4b 03 04 2d 00 00 00  00 00 1b 6e 51 4d 66 82  |PK..-......nQMf.|
// 00000010  13 da ff ff ff ff ff ff  ff ff 08 00 30 00 7a 65  |............0.ze|
// 00000020  72 6f 34 34 30 30 55 54  09 00 03 a5 21 c7 5b db  |ro4400UT....!.[.|
// 00000030  21 c7 5b 75 78 0b 00 01  04 e8 03 00 00 04 e8 03  |!.[ux...........|
// 00000040  00 00 01 00 10 00 00 00  00 13 01 00 00 00 00 00  |................|
// 00000050  00 13 01 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
// 00000060  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
// *
// 113000050  00 00 00 00 00 00 50 4b  03 04 0a 00 00 00 00 00  |......PK........|
// 113000060  2b 6e 51 4d 98 23 28 4b  00 00 40 06 00 00 40 06  |+nQM.#(K..@...@.|
// 113000070  07 00 1c 00 7a 65 72 6f  31 30 30 55 54 09 00 03  |....zero100UT...|
// 113000080  c2 21 c7 5b c2 21 c7 5b  75 78 0b 00 01 04 e8 03  |.!.[.!.[ux......|
// 113000090  00 00 04 e8 03 00 00 00  00 00 00 00 00 00 00 00  |................|
// 1130000a0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
// *
// 119400090  00 00 00 00 00 00 00 50  4b 03 04 2d 00 00 00 00  |.......PK..-....|
// 1194000a0  00 3b 6e 51 4d 66 82 13  da ff ff ff ff ff ff ff  |.;nQMf..........|
// 1194000b0  ff 0a 00 30 00 7a 65 72  6f 34 34 30 30 5f 32 55  |...0.zero4400_2U|
// 1194000c0  54 09 00 03 e2 21 c7 5b  db 21 c7 5b 75 78 0b 00  |T....!.[.!.[ux..|
// 1194000d0  01 04 e8 03 00 00 04 e8  03 00 00 01 00 10 00 00  |................|
// 1194000e0  00 00 13 01 00 00 00 00  00 00 13 01 00 00 00 00  |................|
// 1194000f0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
// *
// 22c4000e0  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 50  |...............P|
// 22c4000f0  4b 01 02 1e 03 2d 00 00  00 00 00 1b 6e 51 4d 66  |K....-......nQMf|
// 22c400100  82 13 da ff ff ff ff ff  ff ff ff 08 00 2c 00 00  |.............,..|
// 22c400110  00 00 00 00 00 00 00 a4  81 00 00 00 00 7a 65 72  |.............zer|
// 22c400120  6f 34 34 30 30 55 54 05  00 03 a5 21 c7 5b 75 78  |o4400UT....!.[ux|
// 22c400130  0b 00 01 04 e8 03 00 00  04 e8 03 00 00 01 00 10  |................|
// 22c400140  00 00 00 00 13 01 00 00  00 00 00 00 13 01 00 00  |................|
// 22c400150  00 50 4b 01 02 1e 03 0a  00 00 00 00 00 2b 6e 51  |.PK..........+nQ|
// 22c400160  4d 98 23 28 4b 00 00 40  06 00 00 40 06 07 00 24  |M.#(K..@...@...$|
// 22c400170  00 00 00 00 00 00 00 00  00 a4 81 ff ff ff ff 7a  |...............z|
// 22c400180  65 72 6f 31 30 30 55 54  05 00 03 c2 21 c7 5b 75  |ero100UT....!.[u|
// 22c400190  78 0b 00 01 04 e8 03 00  00 04 e8 03 00 00 01 00  |x...............|
// 22c4001a0  08 00 56 00 00 13 01 00  00 00 50 4b 01 02 1e 03  |..V.......PK....|
// 22c4001b0  2d 00 00 00 00 00 3b 6e  51 4d 66 82 13 da ff ff  |-.....;nQMf.....|
// 22c4001c0  ff ff ff ff ff ff 0a 00  34 00 00 00 00 00 00 00  |........4.......|
// 22c4001d0  00 00 a4 81 ff ff ff ff  7a 65 72 6f 34 34 30 30  |........zero4400|
// 22c4001e0  5f 32 55 54 05 00 03 e2  21 c7 5b 75 78 0b 00 01  |_2UT....!.[ux...|
// 22c4001f0  04 e8 03 00 00 04 e8 03  00 00 01 00 18 00 00 00  |................|
// 22c400200  00 13 01 00 00 00 00 00  00 13 01 00 00 00 97 00  |................|
// 22c400210  40 19 01 00 00 00 50 4b  06 06 2c 00 00 00 00 00  |@.....PK..,.....|
// 22c400220  00 00 1e 03 2d 00 00 00  00 00 00 00 00 00 03 00  |....-...........|
// 22c400230  00 00 00 00 00 00 03 00  00 00 00 00 00 00 27 01  |..............'.|
// 22c400240  00 00 00 00 00 00 ef 00  40 2c 02 00 00 00 50 4b  |........@,....PK|
// 22c400250  06 07 00 00 00 00 16 02  40 2c 02 00 00 00 01 00  |........@,......|
// 22c400260  00 00 50 4b 05 06 00 00  00 00 03 00 03 00 27 01  |..PK..........'.|
// 22c400270  00 00 ff ff ff ff 00 00                           |........|
// 22c400278
use std::io::{self, Read, Seek, SeekFrom};

const BLOCK1_LENGTH: u64 = 0x60;
const BLOCK1: [u8; BLOCK1_LENGTH as usize] = [
    0x50, 0x4b, 0x03, 0x04, 0x2d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1b, 0x6e, 0x51, 0x4d, 0x66, 0x82,
    0x13, 0xda, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x08, 0x00, 0x30, 0x00, 0x7a, 0x65,
    0x72, 0x6f, 0x34, 0x34, 0x30, 0x30, 0x55, 0x54, 0x09, 0x00, 0x03, 0xa5, 0x21, 0xc7, 0x5b, 0xdb,
    0x21, 0xc7, 0x5b, 0x75, 0x78, 0x0b, 0x00, 0x01, 0x04, 0xe8, 0x03, 0x00, 0x00, 0x04, 0xe8, 0x03,
    0x00, 0x00, 0x01, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x13, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x13, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const BLOCK2_LENGTH: u64 = 0x50;
const BLOCK2: [u8; BLOCK2_LENGTH as usize] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x50, 0x4b, 0x03, 0x04, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x2b, 0x6e, 0x51, 0x4d, 0x98, 0x23, 0x28, 0x4b, 0x00, 0x00, 0x40, 0x06, 0x00, 0x00, 0x40, 0x06,
    0x07, 0x00, 0x1c, 0x00, 0x7a, 0x65, 0x72, 0x6f, 0x31, 0x30, 0x30, 0x55, 0x54, 0x09, 0x00, 0x03,
    0xc2, 0x21, 0xc7, 0x5b, 0xc2, 0x21, 0xc7, 0x5b, 0x75, 0x78, 0x0b, 0x00, 0x01, 0x04, 0xe8, 0x03,
    0x00, 0x00, 0x04, 0xe8, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const BLOCK3_LENGTH: u64 = 0x60;
const BLOCK3: [u8; BLOCK3_LENGTH as usize] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x50, 0x4b, 0x03, 0x04, 0x2d, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x3b, 0x6e, 0x51, 0x4d, 0x66, 0x82, 0x13, 0xda, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0x0a, 0x00, 0x30, 0x00, 0x7a, 0x65, 0x72, 0x6f, 0x34, 0x34, 0x30, 0x30, 0x5f, 0x32, 0x55,
    0x54, 0x09, 0x00, 0x03, 0xe2, 0x21, 0xc7, 0x5b, 0xdb, 0x21, 0xc7, 0x5b, 0x75, 0x78, 0x0b, 0x00,
    0x01, 0x04, 0xe8, 0x03, 0x00, 0x00, 0x04, 0xe8, 0x03, 0x00, 0x00, 0x01, 0x00, 0x10, 0x00, 0x00,
    0x00, 0x00, 0x13, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0x01, 0x00, 0x00, 0x00, 0x00,
];

const BLOCK4_LENGTH: u64 = 0x198;
const BLOCK4: [u8; BLOCK4_LENGTH as usize] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x50,
    0x4b, 0x01, 0x02, 0x1e, 0x03, 0x2d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1b, 0x6e, 0x51, 0x4d, 0x66,
    0x82, 0x13, 0xda, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x08, 0x00, 0x2c, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa4, 0x81, 0x00, 0x00, 0x00, 0x00, 0x7a, 0x65, 0x72,
    0x6f, 0x34, 0x34, 0x30, 0x30, 0x55, 0x54, 0x05, 0x00, 0x03, 0xa5, 0x21, 0xc7, 0x5b, 0x75, 0x78,
    0x0b, 0x00, 0x01, 0x04, 0xe8, 0x03, 0x00, 0x00, 0x04, 0xe8, 0x03, 0x00, 0x00, 0x01, 0x00, 0x10,
    0x00, 0x00, 0x00, 0x00, 0x13, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0x01, 0x00, 0x00,
    0x00, 0x50, 0x4b, 0x01, 0x02, 0x1e, 0x03, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2b, 0x6e, 0x51,
    0x4d, 0x98, 0x23, 0x28, 0x4b, 0x00, 0x00, 0x40, 0x06, 0x00, 0x00, 0x40, 0x06, 0x07, 0x00, 0x24,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa4, 0x81, 0xff, 0xff, 0xff, 0xff, 0x7a,
    0x65, 0x72, 0x6f, 0x31, 0x30, 0x30, 0x55, 0x54, 0x05, 0x00, 0x03, 0xc2, 0x21, 0xc7, 0x5b, 0x75,
    0x78, 0x0b, 0x00, 0x01, 0x04, 0xe8, 0x03, 0x00, 0x00, 0x04, 0xe8, 0x03, 0x00, 0x00, 0x01, 0x00,
    0x08, 0x00, 0x56, 0x00, 0x00, 0x13, 0x01, 0x00, 0x00, 0x00, 0x50, 0x4b, 0x01, 0x02, 0x1e, 0x03,
    0x2d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3b, 0x6e, 0x51, 0x4d, 0x66, 0x82, 0x13, 0xda, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x0a, 0x00, 0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xa4, 0x81, 0xff, 0xff, 0xff, 0xff, 0x7a, 0x65, 0x72, 0x6f, 0x34, 0x34, 0x30, 0x30,
    0x5f, 0x32, 0x55, 0x54, 0x05, 0x00, 0x03, 0xe2, 0x21, 0xc7, 0x5b, 0x75, 0x78, 0x0b, 0x00, 0x01,
    0x04, 0xe8, 0x03, 0x00, 0x00, 0x04, 0xe8, 0x03, 0x00, 0x00, 0x01, 0x00, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x13, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0x01, 0x00, 0x00, 0x00, 0x97, 0x00,
    0x40, 0x19, 0x01, 0x00, 0x00, 0x00, 0x50, 0x4b, 0x06, 0x06, 0x2c, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x1e, 0x03, 0x2d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x01,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xef, 0x00, 0x40, 0x2c, 0x02, 0x00, 0x00, 0x00, 0x50, 0x4b,
    0x06, 0x07, 0x00, 0x00, 0x00, 0x00, 0x16, 0x02, 0x40, 0x2c, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00,
    0x00, 0x00, 0x50, 0x4b, 0x05, 0x06, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x03, 0x00, 0x27, 0x01,
    0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00,
];

const BLOCK1_START: u64 = 0x000000000;
const BLOCK2_START: u64 = 0x113000050;
const BLOCK3_START: u64 = 0x119400090;
const BLOCK4_START: u64 = 0x22c4000e0;

const BLOCK1_END: u64 = BLOCK1_START + BLOCK1_LENGTH - 1;
const BLOCK2_END: u64 = BLOCK2_START + BLOCK2_LENGTH - 1;
const BLOCK3_END: u64 = BLOCK3_START + BLOCK3_LENGTH - 1;
const BLOCK4_END: u64 = BLOCK4_START + BLOCK4_LENGTH - 1;

const TOTAL_LENGTH: u64 = BLOCK4_START + BLOCK4_LENGTH;

struct Zip64File {
    pointer: u64,
}

impl Zip64File {
    fn new() -> Self {
        Zip64File { pointer: 0 }
    }
}

impl Seek for Zip64File {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match pos {
            SeekFrom::Start(offset) => {
                self.pointer = offset;
            }
            SeekFrom::End(offset) => {
                if offset > 0 || offset < -(TOTAL_LENGTH as i64) {
                    return Err(io::Error::new(io::ErrorKind::Other, "Invalid seek offset"));
                }
                self.pointer = (TOTAL_LENGTH as i64 + offset) as u64;
            }
            SeekFrom::Current(offset) => {
                let seekpos = self.pointer as i64 + offset;
                if seekpos < 0 || seekpos as u64 > TOTAL_LENGTH {
                    return Err(io::Error::new(io::ErrorKind::Other, "Invalid seek offset"));
                }
                self.pointer = seekpos as u64;
            }
        }
        Ok(self.pointer)
    }
}

impl Read for Zip64File {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.pointer >= TOTAL_LENGTH {
            return Ok(0);
        }
        match self.pointer {
            BLOCK1_START..=BLOCK1_END => {
                buf[0] = BLOCK1[(self.pointer - BLOCK1_START) as usize];
            }
            BLOCK2_START..=BLOCK2_END => {
                buf[0] = BLOCK2[(self.pointer - BLOCK2_START) as usize];
            }
            BLOCK3_START..=BLOCK3_END => {
                buf[0] = BLOCK3[(self.pointer - BLOCK3_START) as usize];
            }
            BLOCK4_START..=BLOCK4_END => {
                buf[0] = BLOCK4[(self.pointer - BLOCK4_START) as usize];
            }
            _ => {
                buf[0] = 0;
            }
        }
        self.pointer += 1;
        Ok(1)
    }
}

#[test]
fn zip64_large() {
    let zipfile = Zip64File::new();
    let mut archive = zip::ZipArchive::new(zipfile).unwrap();
    let mut buf = [0u8; 32];

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = file.enclosed_name().unwrap();
        println!(
            "Entry {} has name \"{}\" ({} bytes)",
            i,
            outpath.display(),
            file.size()
        );

        file.read_exact(&mut buf).unwrap();
        println!("The first {} bytes are: {:?}", buf.len(), buf);
    }
}

// run this test only when compiled with optimizations
#[cfg(not(debug_assertions))]
mod write {
    use std::io::{Cursor, Read, Write};

    use zip::{write::FileOptions, CompressionMethod, ZipArchive, ZipWriter};

    fn write_large(file: &mut Cursor<Vec<u8>>) -> zip::result::ZipResult<()> {
        let mut zip = ZipWriter::new(file);
        let options = FileOptions::default()
            .compression_method(CompressionMethod::Zstd)
            .compression_level(Some(-7))
            .large_file(true);
        zip.start_file("large", options)?;

        let chunk = vec![0u8; 1 * 1024 * 1024];
        for _ in 0..5000 {
            zip.write_all(&chunk)?;
        }
        zip.finish()?;
        Ok(())
    }

    fn check_large(data: &[u8]) -> zip::result::ZipResult<()> {
        let mut zip = ZipArchive::new(Cursor::new(data))?;
        assert_eq!(zip.file_names().collect::<Vec<_>>(), vec!["large"]);
        let mut file = zip.by_index(0)?;
        assert_eq!(file.size(), 5000 * 1024 * 1024);
        assert!(
            file.compressed_size() < 500_000,
            "expected good compression: {}",
            file.compressed_size(),
        );

        let mut chunk = vec![0u8; 1 * 1024 * 1024];
        let mut total_read = 0;
        loop {
            let bytes_read = file.read(&mut chunk)?;
            total_read += bytes_read;
            for &b in &chunk[..bytes_read] {
                assert_eq!(b, 0);
            }
            if bytes_read == 0 {
                break;
            }
        }
        assert_eq!(total_read, 5000 * 1024 * 1024);

        Ok(())
    }

    fn write_many(file: &mut Cursor<Vec<u8>>) -> zip::result::ZipResult<()> {
        let mut zip = ZipWriter::new(file);
        let options = FileOptions::default().compression_method(CompressionMethod::Stored);

        for i in 0..70_000 {
            zip.start_file(format!("f{:05}", i), options)?;
            zip.write_all(&[0u8; 1])?;
        }
        zip.finish()?;
        Ok(())
    }

    fn check_many(data: &[u8]) -> zip::result::ZipResult<()> {
        let mut zip = ZipArchive::new(Cursor::new(data))?;
        assert_eq!(zip.len(), 70_000);
        for i in 0..70_000 {
            let mut file = zip.by_index(i)?;
            assert_eq!(file.size(), 1);
            let mut buf = [0u8; 1];
            file.read_exact(&mut buf)?;
            assert_eq!(buf[0], 0);
        }

        Ok(())
    }

    #[test]
    fn zip64_large_file_write() {
        let mut buffer = Cursor::new(Vec::new());
        println!("generating zip64 with 5GB file");
        write_large(&mut buffer).unwrap();
        println!("validating zip64 with 5GB file");
        check_large(&buffer.into_inner()).unwrap();
    }

    #[test]
    fn zip64_many_files_write() {
        let mut buffer = Cursor::new(Vec::new());
        println!("generating zip64 with 70k files");
        write_many(&mut buffer).unwrap();
        println!("validating zip64 with 70k files");
        check_many(&buffer.into_inner()).unwrap();
    }
}
