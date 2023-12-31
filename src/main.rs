mod application_json;
mod application_form_encoded;
mod cookies;
mod custom_method;
mod full;
mod headers;
mod https;
mod jsonObj_multiline;
mod jsonObj_null_value;
mod multipart_data;
mod multipart_file;
mod multipart_form_data;
mod multipart_form_data_no_params;
mod nested;
mod query;
mod short;
mod text_plain;

fn main() {
    application_json::main();
    application_form_encoded::main();
    cookies::main();
    custom_method::main();
    full::main();
    headers::main();
    https::main();
    jsonObj_multiline::main();
    jsonObj_null_value::main();
    multipart_data::main();
    multipart_file::main();
    multipart_form_data::main();
    multipart_form_data_no_params::main();
    nested::main();
    query::main();
    short::main();
    text_plain::main();
}
