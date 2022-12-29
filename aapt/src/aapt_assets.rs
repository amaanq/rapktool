const K_ASSET_DIR: &str = "assets";
const K_RESOURCE_DIR: &str = "res";
const K_VALUES_DIR: &str = "values";
const K_MIPMAP_DIR: &str = "mipmap";
const K_INVALID_CHARS: &str = r#"/\\:"#;

const MAX_ASSET_FILE_NAME: usize = 100;

struct AaptAssets {}

enum Axis {
    None = 0,
    MCC = 1,
    MNC,
    LOCALE,
    SCREENLAYOUTSIZE,
    SCREENLAYOUTLONG,
    ORIENTATION,
    UIMODETYPE,
    UIMODENIGHT,
    DENSITY,
    TOUCHSCREEN,
    KEYSHIDDEN,
    KEYBOARD,
    NAVHIDDEN,
    NAVIGATION,
    SCREENSIZE,
    SMALLESTSCREENWIDTHDP,
    SCREENWIDTHDP,
    SCREENHEIGHTDP,
    LAYOUTDIR,
    VERSION,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
struct AaptLocaleValue {
    language: String,
    region: String,
    script: String,
    variant: String,
}

impl AaptLocaleValue {
    fn new() -> Self {
        Self {
            language: String::new(),
            region: String::new(),
            script: String::new(),
            variant: String::new(),
        }
    }
}

// struct AaptGroupEntry
// {
// public:
//     AaptGroupEntry() {}
//     explicit AaptGroupEntry(const ConfigDescription& config) : mParams(config) {}
//     bool initFromDirName(const char* dir, String8* resType);
//     inline const ConfigDescription& toParams() const { return mParams; }
//     inline int compare(const AaptGroupEntry& o) const { return mParams.compareLogical(o.mParams); }
//     inline bool operator<(const AaptGroupEntry& o) const { return compare(o) < 0; }
//     inline bool operator<=(const AaptGroupEntry& o) const { return compare(o) <= 0; }
//     inline bool operator==(const AaptGroupEntry& o) const { return compare(o) == 0; }
//     inline bool operator!=(const AaptGroupEntry& o) const { return compare(o) != 0; }
//     inline bool operator>=(const AaptGroupEntry& o) const { return compare(o) >= 0; }
//     inline bool operator>(const AaptGroupEntry& o) const { return compare(o) > 0; }
//     String8 toString() const { return mParams.toString(); }
//     String8 toDirName(const String8& resType) const;
//     const String8 getVersionString() const { return AaptConfig::getVersion(mParams); }
// private:
//     ConfigDescription mParams;
// };
struct AaptGroupEntry {
    // ConfigDescription mParams;
}

pub fn validate_file_name(file_name: &str) -> bool {
    let mut len = 0;
    for c in file_name.chars() {
        if c as u32 > 0x7f {
            return false;
        }
        if (c as u32) < 0x20 || (c as u32) == 0x7f {
            return false;
        }
        if K_INVALID_CHARS.contains(c) {
            return false;
        }
        len += 1;
    }
    if !(1..=MAX_ASSET_FILE_NAME).contains(&len) {
        return false;
    }
    true
}

// The default to use if no other ignore pattern is defined.
const DEFAULT_IGNORE_ASSETS: &str =
    "!.svn:!.git:!.ds_store:!*.scc:.*:<dir>_*:!CVS:!thumbs.db:!picasa.ini:!*~";

const USER_IGNORE_ASSETS: &str = "";

pub fn is_hidden(root: &str, path: &str) -> bool {
    if path == "." || path == ".." {
        return true;
    }
    let mut ignore = false;
    let mut chatty = true;
    let mut matched_pattern = None;
    let full_path = format!("{}/{}", root, path);
    let file_type = get_file_type(&full_path);
    let plen = path.len();
    let mut patterns = USER_IGNORE_ASSETS.to_string();
    if patterns.is_empty() {
        patterns = DEFAULT_IGNORE_ASSETS.to_string();
    }
    for token in patterns.split(':') {
        chatty = token.starts_with('!');
        if !chatty {
            token = &token[1..];
        }
        if token.starts_with("<dir>") {
            if file_type != FileType::Directory {
                continue;
            }
            token = &token[5..];
        }
        if token.starts_with("<file>") {
            if file_type != FileType::Regular {
                continue;
            }
            token = &token[6..];
        }
        matched_pattern = Some(token);
        let n = token.len();
        if token.starts_with('*') {
            // Match *suffix
            token = &token[1..];
            if n <= plen {
                ignore = token == &path[plen - n..];
            }
        } else if n > 1 && token.ends_with('*') {
            // Match prefix*
            ignore = token == &path[..n - 1];
        } else {
            ignore = token == path;
        }
    }
    if ignore && chatty {
        eprintln!(
            "    (skipping {} '{}' due to ANDROID_AAPT_IGNORE pattern '{}')",
            match file_type {
                FileType::Directory => "dir",
                FileType::Regular => "file",
                _ => "unknown",
            },
            path,
            matched_pattern.unwrap_or("")
        );
    }
    ignore
}
