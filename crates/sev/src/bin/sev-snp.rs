// SPDX-License-Identifier: Apache-2.0

use std::convert::TryInto;
use std::path::PathBuf;
use std::process;

use base64::{engine::general_purpose::STANDARD, Engine};
use clap::{Parser, Subcommand};
use sev::measurement::{
    snp::{get_snp_ovmf_measurement, snp_calc_launch_digest, SnpMeasurementArgs},
    vcpu_types::CpuType,
    vmsa::{GuestFeatures, VMMType},
};

#[derive(Parser)]
#[command(name = "sev-snp", about = "AMD SEV-SNP measurement utilities")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Calculate an SEV-SNP launch digest
    CalcLaunchDigest {
        /// Number of vCPUs
        #[arg(short = 'c', long)]
        vcpus: u32,

        /// vCPU type
        #[arg(short = 't', long, default_value = "epyc-genoa")]
        vcpu_type: String,

        /// Path to OVMF firmware file
        #[arg(short, long)]
        ovmf_file: Option<PathBuf>,

        /// Guest features as a u64 value (default: 0x1, SNPActive only)
        #[arg(short, long, default_value = "0x1")]
        guest_features: String,

        /// Path to kernel file
        #[arg(short, long)]
        kernel_file: Option<PathBuf>,

        /// Path to initrd file
        #[arg(short, long)]
        initrd_file: Option<PathBuf>,

        /// Kernel command-line append string
        #[arg(short, long)]
        append: Option<String>,

        /// Pre-calculated OVMF hash (hex string)
        #[arg(long)]
        ovmf_hash: Option<String>,

        /// VMM type
        #[arg(short, long, default_value = "qemu")]
        vmm_type: String,

        /// Pre-calculated kernel hash (hex string)
        #[arg(long)]
        kernel_hash: Option<String>,

        /// Pre-calculated initrd hash (hex string)
        #[arg(long)]
        initrd_hash: Option<String>,
    },

    /// Extract OVMF measurement (hash + metadata) from a firmware file
    OvmfMeasurement {
        /// Path to OVMF firmware file
        #[arg(short, long)]
        ovmf_file: PathBuf,
    },
}

fn parse_guest_features(s: &str) -> Result<GuestFeatures, String> {
    let val = if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u64::from_str_radix(hex, 16)
            .map_err(|e| format!("invalid guest features hex value: {e}"))?
    } else {
        s.parse::<u64>()
            .map_err(|e| format!("invalid guest features value: {e}"))?
    };
    Ok(GuestFeatures::from(val))
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::CalcLaunchDigest {
            vcpus,
            vcpu_type,
            ovmf_file,
            guest_features,
            kernel_file,
            initrd_file,
            append,
            ovmf_hash,
            vmm_type,
            kernel_hash,
            initrd_hash,
        } => {
            let vcpu_type: CpuType = vcpu_type.as_str().try_into().unwrap_or_else(|e| {
                eprintln!("error: invalid vcpu type '{vcpu_type}': {e}");
                process::exit(1);
            });

            let vmm_type: VMMType = vmm_type.parse().unwrap_or_else(|e| {
                eprintln!("error: invalid vmm type: {e}");
                process::exit(1);
            });

            let guest_features =
                parse_guest_features(&guest_features).unwrap_or_else(|e| {
                    eprintln!("error: {e}");
                    process::exit(1);
                });

            let args = SnpMeasurementArgs {
                vcpus,
                vcpu_type,
                ovmf_file,
                guest_features,
                kernel_file,
                initrd_file,
                append: append.as_deref(),
                ovmf_hash_str: ovmf_hash.as_deref(),
                vmm_type: Some(vmm_type),
                kernel_hash_str: kernel_hash.as_deref(),
                initrd_hash_str: initrd_hash.as_deref(),
                ovmf_measurement: None,
            };

            match snp_calc_launch_digest(args) {
                Ok(digest) => println!("{}", digest.get_hex_ld()),
                Err(e) => {
                    eprintln!("error: failed to calculate launch digest: {e}");
                    process::exit(1);
                }
            }
        }

        Command::OvmfMeasurement { ovmf_file } => {
            match get_snp_ovmf_measurement(ovmf_file) {
                Ok(measurement) => {
                    let bytes = measurement.to_bytes().unwrap_or_else(|e| {
                        eprintln!("error: failed to serialize OVMF measurement: {e}");
                        process::exit(1);
                    });
                    println!("{}", STANDARD.encode(bytes));
                }
                Err(e) => {
                    eprintln!("error: failed to get OVMF measurement: {e}");
                    process::exit(1);
                }
            }
        }
    }
}
