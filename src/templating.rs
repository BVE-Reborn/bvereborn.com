use rocket::fairing::Fairing;
use rocket_contrib::templates::handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError, ScopedJson,
};
use rocket_contrib::templates::Template;
use serde_json::Value;
use std::fs::read_to_string;

macro_rules! register_partial {
    ($handlebars:expr, $name:literal) => {
        $handlebars
            .register_partial(
                $name,
                read_to_string(concat!("templates/partials/", $name, ".hbs")).expect(concat!(
                    "Unable to read file templates/partials/",
                    $name,
                    ".hbs"
                )),
            )
            .expect(concat!("Count not register partial ", $name, ".hbs"))
    };
}

struct StrEq;

impl HelperDef for StrEq {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _: &'reg Handlebars,
        _: &'rc Context,
        _: &mut RenderContext<'reg>,
    ) -> Result<Option<ScopedJson<'reg, 'rc>>, RenderError> {
        // get parameter from helper or throw an error
        let param1 = h.param(0).and_then(|v| v.value().as_str());
        let param2 = h.param(1).and_then(|v| v.value().as_str());

        Ok(Some(ScopedJson::Derived(Value::Bool(
            param1 == param2 && param1.is_some(),
        ))))
    }
}

fn get_link(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    // get parameter from helper or throw an error
    let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    out.write(match param {
        "github" => "https://github.com/BVE-Reborn/bve-reborn",
        "github-website" => "https://github.com/BVE-Reborn/bvereborn.com",
        "openbve" => "https://openbve-project.net",
        "openbve-docs" => "https://openbve-project.net/documentation_hugo/en/",
        _ => panic!("Unknown link name {}", param),
    })?;
    Ok(())
}

pub fn configure_templates() -> impl Fairing {
    Template::custom(|engines| {
        let handlebars = &mut engines.handlebars;
        register_partial!(handlebars, "header");
        register_partial!(handlebars, "navbar");
        register_partial!(handlebars, "page");
        register_partial!(handlebars, "single");
        handlebars.register_helper("link", Box::new(get_link));
        handlebars.register_helper("str_eq", Box::new(StrEq));
    })
}
