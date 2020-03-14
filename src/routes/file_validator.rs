use crate::helpers::Headers;
use bve::parse::mesh::FileType;
use itertools::Itertools;
use rocket::request::Form;
use rocket::{get, post, FromForm, FromFormValue};
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use serde::Serialize;
use std::fmt::Debug;
use std::time::Instant;

#[get("/tools/file-validator")]
pub fn file_validator() -> Headers<Template> {
    Headers::public(Template::render("tools/file-validator", &()))
}

#[derive(FromForm)]
pub struct InputFile {
    contents: String,
    kind: InputKind,
}

#[derive(Debug, FromFormValue)]
pub enum InputKind {
    B3D,
    CSV,
    Animated,
    FunctionScript,
    TrainDat,
    Extensions,
}

#[derive(Serialize)]
pub struct OutputResult {
    error: bool,
    contents: String,
    warnings: String,
    time: f32,
}

fn filter_html(input: impl AsRef<str>) -> String {
    input.as_ref().replace(" ", "&nbsp;").replace("\n", "<br>")
}

fn process_output(result: &(impl Debug + ?Sized), warnings: impl IntoIterator<Item = impl Debug>) -> (String, String) {
    (
        filter_html(format!("{:#?}", result)),
        filter_html(warnings.into_iter().map(|w| format!("{:?}", w)).join("<br>")),
    )
}

#[post("/api/validate-file", data = "<input>")]
pub fn validate_file(input: Form<InputFile>) -> Json<OutputResult> {
    let start = Instant::now();
    let (result, warnings) = match input.kind {
        InputKind::B3D => {
            let parsed = bve::parse::mesh::mesh_from_str(&input.contents, FileType::B3D);
            process_output(&parsed, parsed.errors.clone())
        }
        InputKind::CSV => {
            let parsed = bve::parse::mesh::mesh_from_str(&input.contents, FileType::CSV);
            process_output(&parsed, parsed.errors.clone())
        }
        InputKind::Animated => {
            let (parsed, warnings) = bve::parse::animated::parse_animated_file(&input.contents);
            process_output(&parsed, warnings)
        }
        InputKind::TrainDat => {
            let (parsed, warnings) = bve::parse::train_dat::parse_train_dat(&input.contents);
            process_output(&parsed, warnings)
        }
        InputKind::Extensions => {
            let (parsed, warnings) = bve::parse::extensions_cfg::parse_extensions_cfg(&input.contents);
            process_output(&parsed, warnings)
        }
        InputKind::FunctionScript => {
            let result = bve::parse::function_scripts::parse_function_script(&input.contents);
            match result {
                Ok(("", parsed)) => process_output(&parsed, &<[(); 0]>::from([])),
                Ok((input, parsed)) => process_output(&parsed, &[format!("Remaining unparsed input: {}", input)]),
                Err(e) => process_output("Error", &[format!("{:#?}", e)]),
            }
        }
    };
    let diff = Instant::now() - start;
    Json(OutputResult {
        error: !warnings.is_empty(),
        contents: result,
        warnings,
        time: diff.as_secs_f32(),
    })
}
