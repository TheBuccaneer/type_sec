fn main() {
    #[cfg(feature = "bloat-probe")]
    {
        // benutze den Re-Export
        hpc_core::bloat_hotpath_probe_entry();
    }

    #[cfg(not(feature = "bloat-probe"))]
    {
        // nichts zu tun
    }
}
