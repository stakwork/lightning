fn main() {
    let builder = tonic_build::configure();
    builder
        .protoc_arg("--experimental_allow_proto3_optional")
        .type_attribute(".cln.GetinfoResponse", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.GetinfoOur_features", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.GetinfoAddress", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.GetinfoAddressType", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.GetinfoBinding", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.GetinfoBindingType", "#[derive(serde::Serialize)]")
        .compile(&["proto/node.proto"], &["proto"])
        .unwrap();
}
