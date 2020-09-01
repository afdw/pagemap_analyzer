use bitflags::bitflags;
use byteorder::{LittleEndian, ReadBytesExt};
use lazy_static::lazy_static;
use nix::unistd::{execv, getuid, sysconf, SysconfVar};
use std::{collections::HashMap, env, ffi::CString, fs, io::Cursor, iter};

lazy_static! {
    static ref PAGE_SIZE: usize = sysconf(SysconfVar::PAGE_SIZE).unwrap().unwrap() as usize;
}

bitflags! {
    #[derive(Default)]
    struct PageFlags: u64 {
        const LOCKED = 1 << 0;
        const ERROR = 1 << 1;
        const REFERENCED = 1 << 2;
        const UPTODATE = 1 << 3;
        const DIRTY = 1 << 4;
        const LRU = 1 << 5;
        const ACTIVE = 1 << 6;
        const SLAB = 1 << 7;
        const WRITEBACK = 1 << 8;
        const RECLAIM = 1 << 9;
        const BUDDY = 1 << 10;
        const MMAP = 1 << 11;
        const ANON = 1 << 12;
        const SWAPCACHE = 1 << 13;
        const SWAPBACKED = 1 << 14;
        const COMPOUND_HEAD = 1 << 15;
        const COMPOUND_TAIL = 1 << 16;
        const HUGE = 1 << 17;
        const UNEVICTABLE = 1 << 18;
        const HWPOISON = 1 << 19;
        const NOPAGE = 1 << 20;
        const KSM = 1 << 21;
        const THP = 1 << 22;
        const BALLOON = 1 << 23;
        const ZERO_PAGE = 1 << 24;
        const IDLE = 1 << 25;
        const KPF_PGTABLE = 1 << 26;
        const RESERVED = 1 << 32;
        const MLOCKED = 1 << 33;
        const MAPPEDTODISK = 1 << 34;
        const PRIVATE = 1 << 35;
        const PRIVATE_2 = 1 << 36;
        const OWNER_PRIVATE = 1 << 37;
        const ARCH = 1 << 38;
        const UNCACHED = 1 << 39;
        const SOFTDIRTY = 1 << 40;
        const READAHEAD = 1 << 48;
        const SLOB_FREE = 1 << 49;
        const SLUB_FROZEN = 1 << 50;
        const SLUB_DEBUG = 1 << 51;
        const FILE = 1 << 61;
        const SWAP = 1 << 62;
        const MMAP_EXCLUSIVE = 1 << 63;
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct Page {
    times_mapped: u64,
    cgroup_inode: u64,
    flags: PageFlags,
}

fn become_root() {
    if !getuid().is_root() {
        let args = iter::once("/usr/bin/sudo".into())
            .chain(env::args())
            .map(|a| CString::new(a).unwrap())
            .collect::<Vec<_>>();
        execv(
            &CString::new("/usr/bin/sudo").unwrap(),
            &args.iter().map(|a| a.as_ref()).collect::<Vec<_>>()[..],
        )
        .unwrap();
    }
}

fn page_count_to_size_string(page_count: usize) -> String {
    byte_unit::Byte::from_bytes((page_count * *PAGE_SIZE) as u128)
        .get_appropriate_unit(true)
        .to_string()
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    become_root();
    let kpagecount_read = fs::read("/proc/kpagecount")?;
    let kpagecgroup_read = fs::read("/proc/kpagecgroup")?;
    let kpageflags_read = fs::read("/proc/kpageflags")?;
    let page_count = *[
        kpagecount_read.len() / 8,
        kpagecgroup_read.len() / 8,
        kpageflags_read.len() / 8,
    ]
    .iter()
    .min()
    .unwrap();
    let mut pages = iter::repeat(Page::default())
        .take(page_count)
        .collect::<Vec<_>>();
    {
        let mut kpagecount_read = Cursor::new(&kpagecount_read);
        let mut kpagecgroup_read = Cursor::new(&kpagecgroup_read);
        let mut kpageflags_read = Cursor::new(&kpageflags_read);
        for (i, page) in pages.iter_mut().enumerate() {
            page.times_mapped = kpagecount_read.read_u64::<LittleEndian>()?;
            page.cgroup_inode = kpagecgroup_read.read_u64::<LittleEndian>()?;
            let flags_bits = kpageflags_read.read_u64::<LittleEndian>()?;
            page.flags = PageFlags::from_bits(flags_bits)
                .ok_or(format!("wrong flags: {:#b} at PFN {}", flags_bits, i))?;
        }
    }
    let mut flags_counts = HashMap::<_, usize>::new();
    for page in &pages {
        *flags_counts.entry(page.flags).or_default() += 1;
    }
    let mut sorted_flags_counts = flags_counts.clone().into_iter().collect::<Vec<_>>();
    sorted_flags_counts.sort_by_key(|(_, count)| !*count);
    for (i, &(page_flags, count)) in sorted_flags_counts.iter().enumerate().take(15) {
        println!(
            "{:>5}. {:>11} - {:?}",
            i + 1,
            page_count_to_size_string(count),
            page_flags
        );
    }
    println!(
        "Total: {:>11}",
        page_count_to_size_string(
            pages
                .iter()
                .filter(|page| !page.flags.intersects(PageFlags::NOPAGE))
                .count()
        )
    );
    Ok(())
}
