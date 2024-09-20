fn main() {
    use build_script_cfg::Cfg;

    let common_cpu = Cfg::new("common_cpu");
    let nvidia_gpu = Cfg::new("nvidia_gpu");

    if cfg!(feature = "common-cpu") {
        common_cpu.define();
    }
    if cfg!(feature = "nvidia-gpu") && search_cuda_tools::find_cuda_root().is_some() {
        nvidia_gpu.define();
    }
}
