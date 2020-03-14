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
    errors: u64,
    contents: String,
    warnings: String,
    time: f32,
}

fn filter_html(input: impl AsRef<str>) -> String {
    input.as_ref().replace(" ", "&nbsp;").replace("\n", "<br>")
}

fn process_output(
    result: &(impl Debug + ?Sized),
    warnings: impl IntoIterator<Item = impl Debug>,
) -> (String, String, u64) {
    let result_string = filter_html(format!("{:#?}", result));
    let mut count = 0_u64;
    let warning_string = warnings
        .into_iter()
        .map(|w| {
            count += 1;
            format!("{:?}", w)
        })
        .join("<br>");
    (result_string, warning_string, count)
}

#[post("/api/validate-file", data = "<input>")]
pub fn validate_file(input: Form<InputFile>) -> Json<OutputResult> {
    let start = Instant::now();
    let (result, warnings, count) = match input.kind {
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
        errors: count,
        contents: result,
        warnings,
        time: diff.as_secs_f32(),
    })
}
