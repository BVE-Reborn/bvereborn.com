use crate::helpers::{Headers, Os};
use rocket::get;
use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
struct DownloadCtx {
    os: String,
    version: &'static str,
    preferred: DownloadType,
    others: Vec<DownloadType>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct DownloadType {
    os: Os,
    name: &'static str,
    link: &'static str,
}

#[get("/download")]
pub fn download(request_os: Os) -> Headers<Template> {
    let mut installers = vec![
        DownloadType {
            os: Os::Windows,
            name: "Windows Installer, .msi, x64, 128MB",
            link: "#wininst",
        },
        DownloadType {
            os: Os::Mac,
            name: "Mac Installer, .dmg, x64, 128MB",
            link: "#macinst",
        },
        DownloadType {
            os: Os::Linux,
            name: "Linux Debian Package, .deb, x64, 128MB",
            link: "#deb",
        },
    ];
    let archives = vec![
        DownloadType {
            os: Os::Windows,
            name: "Windows Installer, .zip, x64, 128MB",
            link: "#winarc",
        },
        DownloadType {
            os: Os::Mac,
            name: "Mac Archive, .zip, x64, 128MB",
            link: "#macarc",
        },
        DownloadType {
            os: Os::Linux,
            name: "Linux Archive, .tar.gz, x64, 128MB",
            link: "#linarc",
        },
    ];

    // This will put our preferred at the end
    installers.sort_by_key(|DownloadType { os, .. }| *os == request_os);
    // We pop off our preferred
    let preferred = installers.pop().expect("Must have value");

    let mut others = installers;
    others.extend_from_slice(&archives);

    others.sort_by_key(|DownloadType { os, .. }| (*os != request_os, *os));

    let ctx = DownloadCtx {
        os: format!("{:?}", request_os),
        version: "0.0.1",
        preferred,
        others,
    };
    Headers::public(Template::render("download", &ctx))
}
