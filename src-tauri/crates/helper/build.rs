fn main() {
    #[cfg(windows)]
    {
        println!("cargo:rerun-if-changed=helper.rc");
        println!("cargo:rerun-if-changed=helper.manifest");

        // `manifest_required`, not `manifest_optional`: without the embedded
        // manifest this binary would launch unelevated and every privileged
        // call would fail at runtime. A build error is far better than a
        // binary that looks fine and silently cannot do its job.
        embed_resource::compile("helper.rc", embed_resource::NONE)
            .manifest_required()
            .expect("failed to embed the helper manifest");
    }
}
