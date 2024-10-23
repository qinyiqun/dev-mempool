fn main() {
    use build_script_cfg::Cfg;

    let common_cpu = Cfg::new("common_cpu");
    let nvidia_gpu = Cfg::new("nvidia_gpu");
    let mthreads_gpu = Cfg::new("mthreads_gpu");

    if cfg!(feature = "common-cpu") {
        common_cpu.define();
    }
    if cfg!(feature = "nvidia-gpu") && search_cuda_tools::find_cuda_root().is_some() {
        nvidia_gpu.define();
    }
    if cfg!(feature = "mthreads-gpu") && search_musa_tools::find_musa_home().is_some() {
        mthreads_gpu.define();
    }
}
