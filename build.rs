use npm_rs::*;

fn main() {
    NpmEnv::default()
        .with_node_env(&NodeEnv::from_cargo_profile().unwrap_or_default())
        .set_path("webui")
        .init_env()
        .install(None)
        .run("build")
        .exec()
        .unwrap();

    build_deps::rerun_if_changed_paths("build.rs").unwrap();
    build_deps::rerun_if_changed_paths("webui/package.json").unwrap();
    build_deps::rerun_if_changed_paths("webui/vite.config.ts").unwrap();
    build_deps::rerun_if_changed_paths("webui/src/*").unwrap();
}
