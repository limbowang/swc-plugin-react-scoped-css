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
        println!("{}", input.display().to_string());
        test_fixture(
            syntax(),
            &|_| react_scoped_css(Config::default(), input.display().to_string()),
            &input,
            &output,
            Default::default(),
        );
    }
}
