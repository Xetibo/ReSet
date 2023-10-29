fn main() {
    glib_build_tools::compile_resources(
        &["src/resources"],
        "src/resources/resources.gresource.xml",
        "src.templates.gresource",
    );
    glib_build_tools::compile_resources(
        &["src/resources/icons"],
        "src/resources/icons/resources.gresource.xml",
        "src.icons.gresource",
    );
}
