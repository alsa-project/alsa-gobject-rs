// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use alsactl_sys::*;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::mem::{align_of, size_of};
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["alsactl"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For _Generic
        args.push("-std=c11".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Self { args })
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {:?} failed, {}", &cmd, status).into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{} {}", name, err).into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let pkg_config = env::var_os("PKG_CONFIG").unwrap_or_else(|| OsString::from("pkg-config"));
    let mut cmd = Command::new(pkg_config);
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}", &cmd, out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn summary(&self) -> String {
        format!("{} passed; {} failed", self.passed, self.failed)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
#[cfg(target_os = "linux")]
fn cross_validate_constants_with_c() {
    let mut c_constants: Vec<(String, String)> = Vec::new();

    for l in get_c_output("constant").unwrap().lines() {
        let mut words = l.trim().split(';');
        let name = words.next().expect("Failed to parse name").to_owned();
        let value = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse value");
        c_constants.push((name, value));
    }

    let mut results = Results::default();

    for ((rust_name, rust_value), (c_name, c_value)) in
        RUST_CONSTANTS.iter().zip(c_constants.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_value != c_value {
            results.record_failed();
            eprintln!(
                "Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_value, &c_value
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

#[test]
#[cfg(target_os = "linux")]
fn cross_validate_layout_with_c() {
    let mut c_layouts = Vec::new();

    for l in get_c_output("layout").unwrap().lines() {
        let mut words = l.trim().split(';');
        let name = words.next().expect("Failed to parse name").to_owned();
        let size = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse size");
        let alignment = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse alignment");
        c_layouts.push((name, Layout { size, alignment }));
    }

    let mut results = Results::default();

    for ((rust_name, rust_layout), (c_name, c_layout)) in RUST_LAYOUTS.iter().zip(c_layouts.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_layout != c_layout {
            results.record_failed();
            eprintln!(
                "Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_layout, &c_layout
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

fn get_c_output(name: &str) -> Result<String, Box<dyn Error>> {
    let tmpdir = Builder::new().prefix("abi").tempdir()?;
    let exe = tmpdir.path().join(name);
    let c_file = Path::new("tests").join(name).with_extension("c");

    let cc = Compiler::new().expect("configured compiler");
    cc.compile(&c_file, &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}", &abi_cmd, &output).into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    (
        "ALSACtlCard",
        Layout {
            size: size_of::<ALSACtlCard>(),
            alignment: align_of::<ALSACtlCard>(),
        },
    ),
    (
        "ALSACtlCardClass",
        Layout {
            size: size_of::<ALSACtlCardClass>(),
            alignment: align_of::<ALSACtlCardClass>(),
        },
    ),
    (
        "ALSACtlCardError",
        Layout {
            size: size_of::<ALSACtlCardError>(),
            alignment: align_of::<ALSACtlCardError>(),
        },
    ),
    (
        "ALSACtlCardInfo",
        Layout {
            size: size_of::<ALSACtlCardInfo>(),
            alignment: align_of::<ALSACtlCardInfo>(),
        },
    ),
    (
        "ALSACtlCardInfoClass",
        Layout {
            size: size_of::<ALSACtlCardInfoClass>(),
            alignment: align_of::<ALSACtlCardInfoClass>(),
        },
    ),
    (
        "ALSACtlElemAccessFlag",
        Layout {
            size: size_of::<ALSACtlElemAccessFlag>(),
            alignment: align_of::<ALSACtlElemAccessFlag>(),
        },
    ),
    (
        "ALSACtlElemEventMask",
        Layout {
            size: size_of::<ALSACtlElemEventMask>(),
            alignment: align_of::<ALSACtlElemEventMask>(),
        },
    ),
    (
        "ALSACtlElemIfaceType",
        Layout {
            size: size_of::<ALSACtlElemIfaceType>(),
            alignment: align_of::<ALSACtlElemIfaceType>(),
        },
    ),
    (
        "ALSACtlElemInfoBoolean",
        Layout {
            size: size_of::<ALSACtlElemInfoBoolean>(),
            alignment: align_of::<ALSACtlElemInfoBoolean>(),
        },
    ),
    (
        "ALSACtlElemInfoBooleanClass",
        Layout {
            size: size_of::<ALSACtlElemInfoBooleanClass>(),
            alignment: align_of::<ALSACtlElemInfoBooleanClass>(),
        },
    ),
    (
        "ALSACtlElemInfoBytes",
        Layout {
            size: size_of::<ALSACtlElemInfoBytes>(),
            alignment: align_of::<ALSACtlElemInfoBytes>(),
        },
    ),
    (
        "ALSACtlElemInfoBytesClass",
        Layout {
            size: size_of::<ALSACtlElemInfoBytesClass>(),
            alignment: align_of::<ALSACtlElemInfoBytesClass>(),
        },
    ),
    (
        "ALSACtlElemInfoCommonInterface",
        Layout {
            size: size_of::<ALSACtlElemInfoCommonInterface>(),
            alignment: align_of::<ALSACtlElemInfoCommonInterface>(),
        },
    ),
    (
        "ALSACtlElemInfoEnumerated",
        Layout {
            size: size_of::<ALSACtlElemInfoEnumerated>(),
            alignment: align_of::<ALSACtlElemInfoEnumerated>(),
        },
    ),
    (
        "ALSACtlElemInfoEnumeratedClass",
        Layout {
            size: size_of::<ALSACtlElemInfoEnumeratedClass>(),
            alignment: align_of::<ALSACtlElemInfoEnumeratedClass>(),
        },
    ),
    (
        "ALSACtlElemInfoIec60958",
        Layout {
            size: size_of::<ALSACtlElemInfoIec60958>(),
            alignment: align_of::<ALSACtlElemInfoIec60958>(),
        },
    ),
    (
        "ALSACtlElemInfoIec60958Class",
        Layout {
            size: size_of::<ALSACtlElemInfoIec60958Class>(),
            alignment: align_of::<ALSACtlElemInfoIec60958Class>(),
        },
    ),
    (
        "ALSACtlElemInfoInteger",
        Layout {
            size: size_of::<ALSACtlElemInfoInteger>(),
            alignment: align_of::<ALSACtlElemInfoInteger>(),
        },
    ),
    (
        "ALSACtlElemInfoInteger64",
        Layout {
            size: size_of::<ALSACtlElemInfoInteger64>(),
            alignment: align_of::<ALSACtlElemInfoInteger64>(),
        },
    ),
    (
        "ALSACtlElemInfoInteger64Class",
        Layout {
            size: size_of::<ALSACtlElemInfoInteger64Class>(),
            alignment: align_of::<ALSACtlElemInfoInteger64Class>(),
        },
    ),
    (
        "ALSACtlElemInfoIntegerClass",
        Layout {
            size: size_of::<ALSACtlElemInfoIntegerClass>(),
            alignment: align_of::<ALSACtlElemInfoIntegerClass>(),
        },
    ),
    (
        "ALSACtlElemInfoSingleArrayInterface",
        Layout {
            size: size_of::<ALSACtlElemInfoSingleArrayInterface>(),
            alignment: align_of::<ALSACtlElemInfoSingleArrayInterface>(),
        },
    ),
    (
        "ALSACtlElemType",
        Layout {
            size: size_of::<ALSACtlElemType>(),
            alignment: align_of::<ALSACtlElemType>(),
        },
    ),
    (
        "ALSACtlElemValue",
        Layout {
            size: size_of::<ALSACtlElemValue>(),
            alignment: align_of::<ALSACtlElemValue>(),
        },
    ),
    (
        "ALSACtlElemValueClass",
        Layout {
            size: size_of::<ALSACtlElemValueClass>(),
            alignment: align_of::<ALSACtlElemValueClass>(),
        },
    ),
    (
        "ALSACtlEventType",
        Layout {
            size: size_of::<ALSACtlEventType>(),
            alignment: align_of::<ALSACtlEventType>(),
        },
    ),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(gint) ALSACTL_CARD_ERROR_DISCONNECTED", "1"),
    ("(gint) ALSACTL_CARD_ERROR_ELEM_EXIST", "5"),
    ("(gint) ALSACTL_CARD_ERROR_ELEM_NOT_FOUND", "2"),
    ("(gint) ALSACTL_CARD_ERROR_ELEM_NOT_SUPPORTED", "3"),
    ("(gint) ALSACTL_CARD_ERROR_ELEM_OWNED", "4"),
    ("(gint) ALSACTL_CARD_ERROR_FAILED", "0"),
    ("(guint) ALSACTL_ELEM_ACCESS_FLAG_INACTIVE", "256"),
    ("(guint) ALSACTL_ELEM_ACCESS_FLAG_LOCK", "512"),
    ("(guint) ALSACTL_ELEM_ACCESS_FLAG_OWNER", "1024"),
    ("(guint) ALSACTL_ELEM_ACCESS_FLAG_READ", "1"),
    ("(guint) ALSACTL_ELEM_ACCESS_FLAG_TLV_CALLBACK", "268435456"),
    ("(guint) ALSACTL_ELEM_ACCESS_FLAG_TLV_COMMAND", "64"),
    ("(guint) ALSACTL_ELEM_ACCESS_FLAG_TLV_READ", "16"),
    ("(guint) ALSACTL_ELEM_ACCESS_FLAG_TLV_WRITE", "32"),
    ("(guint) ALSACTL_ELEM_ACCESS_FLAG_USER", "536870912"),
    ("(guint) ALSACTL_ELEM_ACCESS_FLAG_VOLATILE", "4"),
    ("(guint) ALSACTL_ELEM_ACCESS_FLAG_WRITE", "2"),
    ("(guint) ALSACTL_ELEM_EVENT_MASK_ADD", "4"),
    ("(guint) ALSACTL_ELEM_EVENT_MASK_INFO", "2"),
    ("(guint) ALSACTL_ELEM_EVENT_MASK_REMOVE", "16"),
    ("(guint) ALSACTL_ELEM_EVENT_MASK_TLV", "8"),
    ("(guint) ALSACTL_ELEM_EVENT_MASK_VALUE", "1"),
    ("(gint) ALSACTL_ELEM_IFACE_TYPE_CARD", "0"),
    ("(gint) ALSACTL_ELEM_IFACE_TYPE_HWDEP", "1"),
    ("(gint) ALSACTL_ELEM_IFACE_TYPE_MIXER", "2"),
    ("(gint) ALSACTL_ELEM_IFACE_TYPE_PCM", "3"),
    ("(gint) ALSACTL_ELEM_IFACE_TYPE_RAWMIDI", "4"),
    ("(gint) ALSACTL_ELEM_IFACE_TYPE_SEQUENCER", "6"),
    ("(gint) ALSACTL_ELEM_IFACE_TYPE_TIMER", "5"),
    ("(gint) ALSACTL_ELEM_TYPE_BOOLEAN", "1"),
    ("(gint) ALSACTL_ELEM_TYPE_BYTES", "4"),
    ("(gint) ALSACTL_ELEM_TYPE_ENUMERATED", "3"),
    ("(gint) ALSACTL_ELEM_TYPE_IEC60958", "5"),
    ("(gint) ALSACTL_ELEM_TYPE_INTEGER", "2"),
    ("(gint) ALSACTL_ELEM_TYPE_INTEGER64", "6"),
    ("(gint) ALSACTL_ELEM_TYPE_NONE", "0"),
    ("(gint) ALSACTL_EVENT_TYPE_ELEM", "0"),
];
