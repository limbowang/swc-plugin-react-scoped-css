use regex::Regex;
use serde::{Deserialize, Serialize};
use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{Ident, JSXAttr, JSXAttrName, JSXAttrValue, Lit},
        visit::{as_folder, Fold, VisitMut, VisitMutWith},
    },
};

use crate::hash::compute_hash;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default)]
    pub hash_seed: String,

    #[serde(with = "serde_regex", default = "default_include")]
    pub include: Regex,
}

fn default_include() -> Regex {
    Regex::new(r".scoped\.(le|sc|sa|c)ss$").unwrap()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            include: default_include(),
            hash_seed: "".to_string(),
        }
    }
}

pub struct ReactScopedCssVistor {
    pub config: Config,
    pub filename: String,
    has_scoped_css: bool,
}

impl Default for ReactScopedCssVistor {
    fn default() -> Self {
        Self {
            config: Config {
                include: default_include(),
                hash_seed: "".to_string(),
            },
            filename: "".to_string(),
            has_scoped_css: false,
        }
    }
}

impl VisitMut for ReactScopedCssVistor {
    fn visit_mut_import_decl(&mut self, import: &mut swc_core::ecma::ast::ImportDecl) {
        if self.config.include.is_match(&import.src.value) {
            self.has_scoped_css = true;
            let hash = compute_hash(&self.config.hash_seed, &self.filename);
            import.src = Box::new(format!("{}?scopeId={}", import.src.value, hash).into());
        }
        import.visit_mut_children_with(self);
    }

    fn visit_mut_jsx_element(&mut self, element: &mut swc_core::ecma::ast::JSXElement) {
        if self.has_scoped_css {
            match element.opening.name {
                swc_core::ecma::ast::JSXElementName::JSXMemberExpr(ref member_expr) => {
                    return;
                }
                _ => {}
            }
            let hash = compute_hash(&self.config.hash_seed, &self.filename);
            element
                .opening
                .attrs
                .push(swc_core::ecma::ast::JSXAttrOrSpread::JSXAttr(JSXAttr {
                    span: DUMMY_SP,
                    name: JSXAttrName::Ident(Ident {
                        span: DUMMY_SP,
                        sym: format!("data-v-{}", hash).into(),
                        optional: false,
                    }),
                    value: JSXAttrValue::Lit(Lit::Str("".into())).into(),
                }))
        }

        element.visit_mut_children_with(self);
    }
}

pub fn react_scoped_css(config: Config, filename: String) -> impl Fold {
    as_folder(ReactScopedCssVistor {
        config,
        filename,
        has_scoped_css: false,
    })
}
