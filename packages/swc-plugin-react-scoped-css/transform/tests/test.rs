#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use react_scoped_css::visitor::{react_scoped_css, Config};
    use swc_ecma_parser::{EsConfig, Syntax};
    use swc_ecma_transforms_testing::test_fixture;
    use testing::fixture;

    fn syntax() -> Syntax {
        Syntax::Es(EsConfig {
            jsx: true,
            ..Default::default()
        })
    }

    #[fixture("tests/fixtures/**/input.jsx")]
    fn tests(input: PathBuf) {
        let output = input.parent().unwrap().join("output.jsx");

        let file_name = input.to_str().unwrap().to_string();
        let current_dir = std::env::current_dir().unwrap();
        let file_path = current_dir.join(file_name.to_string());
        let relative_path = file_path
            .strip_prefix(&current_dir)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        println!("{}", relative_path.clone());

        test_fixture(
            syntax(),
            &|_| react_scoped_css(Config::default(), relative_path.clone()),
            &input,
            &output,
            Default::default(),
        );
    }
}
