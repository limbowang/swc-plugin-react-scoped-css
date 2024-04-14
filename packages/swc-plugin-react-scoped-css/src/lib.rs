use react_scoped_css::visitor::{react_scoped_css, Config};
use swc_core::common::FileName;
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_core::{
    ecma::{ast::Program, visit::FoldWith},
    plugin::metadata::TransformPluginMetadataContextKind,
};

#[plugin_transform]
pub fn process_transform(program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<Config>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for react-scoped-css"),
    )
    .expect("invalid config for react-scoped-css");

    let file_name = match data.get_context(&TransformPluginMetadataContextKind::Filename) {
        Some(s) => FileName::Real(s.into()),
        None => FileName::Anon,
    };
    program.fold_with(&mut react_scoped_css(config, file_name.to_string()))
}
