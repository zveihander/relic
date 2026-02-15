use std::fs::File;
use std::io::Read;
use std::mem;

const IW_ESSID_MAX_SIZE: usize = 32;

// NOTE: Just a small helper to parse ESSIDs

fn parse_ssid(buf: &[u8], len: usize) -> String {
    let actual_len = len.min(buf.len());
    if let Ok(parsed) = std::str::from_utf8(&buf[..actual_len]) {
        let trimmed = parsed.split('\0').next().unwrap_or("").trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    "Disconnected".to_string()
}

// NOTE: Im gonna be 100% honest i got really confused with all the unsafe libc bs here so I used a lot of AI for this.

#[cfg(feature = "wifi_essid")]
pub fn wifi_essid(interface: &str) -> String {
    unsafe {
        let fd = libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0);
        if fd < 0 {
            return "n/a".into();
        }

        let mut ssid_buf = [0u8; IW_ESSID_MAX_SIZE + 1];
        let mut ifr: libc::ifreq = mem::zeroed();

        let if_bytes = interface.as_bytes();
        let len = if_bytes.len().min(libc::IFNAMSIZ - 1);
        std::ptr::copy_nonoverlapping(if_bytes.as_ptr(), ifr.ifr_name.as_mut_ptr() as *mut u8, len);

        #[repr(C)]
        struct IwPoint {
            pointer: *mut libc::c_void,
            length: u16,
            flags: u16,
        }

        let mut data = IwPoint {
            pointer: ssid_buf.as_mut_ptr() as *mut libc::c_void,
            length: IW_ESSID_MAX_SIZE as u16,
            flags: 0,
        };

        std::ptr::copy_nonoverlapping(
            &data as *const _ as *const u8,
            (&mut ifr.ifr_ifru as *mut _) as *mut u8,
            mem::size_of::<IwPoint>(),
        );

        const SIOCGIWESSID: libc::c_ulong = 0x8B1B;
        if libc::ioctl(fd, SIOCGIWESSID, &mut ifr) < 0 {
            libc::close(fd);
            return "Disconnected".into();
        }

        libc::close(fd);

        parse_ssid(&ssid_buf, data.length as usize)
    }
}

#[cfg(feature = "wifi_perc")]
pub fn wifi_perc(interface: &str) -> String {
    let mut buf = [0u8; 1024];
    let mut file = match File::open("/proc/net/wireless") {
        Ok(f) => f,
        Err(_) => return "0".into(),
    };

    let n = file.read(&mut buf).unwrap_or(0);
    let s = std::str::from_utf8(&buf[..n]).unwrap_or("");

    for line in s.lines() {
        if line.contains(interface) {
            let mut parts = line.split_whitespace();
            let _iface = parts.next();
            let _status = parts.next();
            if let Some(quality_str) = parts.next() {
                let quality = quality_str
                    .trim_end_matches('.')
                    .parse::<f32>()
                    .unwrap_or(0.0);

                let perc = (quality / 70.0) * 100.0;
                return format!("{:.0}", perc.clamp(0.0, 100.0));
            }
        }
    }
    "0".into()
}

#[cfg(feature = "wifi_custom")]
pub fn wifi_custom(interface: &str) -> String {
    let mut perc = 0;
    let mut buf = [0u8; 1024];
    if let Ok(mut file) = File::open("/proc/net/wireless")
        && let Ok(n) = file.read(&mut buf)
    {
        let s = std::str::from_utf8(&buf[..n]).unwrap_or("");
        for line in s.lines() {
            if line.contains(interface) {
                let mut parts = line.split_whitespace();
                let _ = parts.next();
                let _ = parts.next();
                if let Some(q_str) = parts.next() {
                    let q = q_str.trim_end_matches('.').parse::<f32>().unwrap_or(0.0);
                    perc = ((q / 70.0) * 100.0).clamp(0.0, 100.0) as u8;
                }
                break;
            }
        }
    }

    let mut ssid = String::from("disconnected");
    unsafe {
        let fd = libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0);
        if fd >= 0 {
            let mut ssid_buf = [0u8; IW_ESSID_MAX_SIZE + 1];
            let mut ifr: libc::ifreq = mem::zeroed();
            let if_bytes = interface.as_bytes();
            let len = if_bytes.len().min(libc::IFNAMSIZ - 1);
            std::ptr::copy_nonoverlapping(if_bytes.as_ptr(), ifr.ifr_name.as_mut_ptr() as *mut u8, len);

            #[repr(C)]
            struct IwPoint {
                pointer: *mut libc::c_void,
                length: u16,
                flags: u16,
            }

            let mut data = IwPoint {
                pointer: ssid_buf.as_mut_ptr() as *mut libc::c_void,
                length: IW_ESSID_MAX_SIZE as u16,
                flags: 0,
            };

            std::ptr::copy_nonoverlapping(
                &data as *const _ as *const u8,
                (&mut ifr.ifr_ifru as *mut _) as *mut u8,
                mem::size_of::<IwPoint>(),
            );

            const SIOCGIWESSID: libc::c_ulong = 0x8B1B;
            if libc::ioctl(fd, SIOCGIWESSID, &mut ifr) >= 0 {
                let res = parse_ssid(&ssid_buf, data.length as usize);
                if res != "Disconnected" {
                    ssid = res;
                }
            }
            libc::close(fd);
        }
    }

    let icon = if ssid == "disconnected" {
        "󰤮 "
    } else {
        match perc {
            80..=100 => "󰤨 ",
            60..=79 => "󰤥 ",
            40..=59 => "󰤢 ",
            20..=39 => "󰤟 ",
            _ => "󰤯 ",
        }
    };

    format!("{} {} ({}%)", icon, ssid, perc)
}
