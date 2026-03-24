use pelite::pe64::{Pe, PeFile};
use std::fs;
use std::path::{PathBuf};

pub fn pattern_to_bytes(pattern: &str) -> Vec<Option<u8>> {
    pattern
        .split_whitespace()
        .map(|token| {
            if token == "?" {
                None
            } else {
                u8::from_str_radix(token, 16).ok()
            }
        })
        .collect()
}

pub fn match_pattern(data: &[u8], pattern: &[Option<u8>], index: usize) -> bool {
    if index + pattern.len() > data.len() {
        return false;
    }
    for (i, p) in pattern.iter().enumerate() {
        if let Some(byte) = p {
            if data[index + i] != *byte {
                return false;
            }
        }
    }
    true
}

pub fn file_offset_to_rva(offset: u32, pe: &PeFile) -> Option<u32> {
    for section in pe.section_headers() {
        let start = section.PointerToRawData;
        let end = start + section.SizeOfRawData;
        if offset >= start && offset < end {
            return Some(offset - start + section.VirtualAddress);
        }
    }
    None
}

pub fn find_qword_addr(data: &[u8], base_addr: u64, pe: &PeFile) -> Option<usize> {
    let pattern = pattern_to_bytes("48 8B 05 ? ? ? ? 48 8D 0D ? ? ? ? FF D0");

    for i in 0..=data.len().saturating_sub(pattern.len()) {
        if match_pattern(data, &pattern, i) {
            let rva = file_offset_to_rva(i as u32, pe)?;
            let va = base_addr + rva as u64;

            let rel_offset = i + 3;
            if rel_offset + 4 > data.len() {
                continue;
            }

            let disp = i32::from_le_bytes(data[rel_offset..rel_offset + 4].try_into().unwrap());
            let rip_after_mov = va + 7;
            let qword_addr = rip_after_mov.wrapping_add(disp as u64);
            return Some((qword_addr - base_addr) as usize);
        }
    }

    None
}

pub fn scan_unity_player(unity_player_path: PathBuf) -> Option<usize> {
    let base_addr = 0x1800_0000_00;

    let data = fs::read(&unity_player_path).ok()?;
    let pe = PeFile::from_bytes(&data).ok()?; 

    find_qword_addr(&data, base_addr, &pe)
}