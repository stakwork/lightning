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
        .type_attribute(".cln.Amount", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.ListpeersResponse", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.ListpeersPeers", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.ListpeersPeersLog", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.ListpeersPeersChannels", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.ListpeersPeersLogType", "#[derive(serde::Serialize)]")
        .type_attribute(
            ".cln.ListpeersPeersChannelsState",
            "#[derive(serde::Serialize)]",
        )
        .type_attribute(
            ".cln.ListpeersPeersChannelsFeerate",
            "#[derive(serde::Serialize)]",
        )
        .type_attribute(
            ".cln.ListpeersPeersChannelsInflight",
            "#[derive(serde::Serialize)]",
        )
        .type_attribute(".cln.ChannelSide", "#[derive(serde::Serialize)]")
        .type_attribute(
            ".cln.ListpeersPeersChannelsFunding",
            "#[derive(serde::Serialize)]",
        )
        .type_attribute(
            ".cln.ListpeersPeersChannelsAlias",
            "#[derive(serde::Serialize)]",
        )
        .type_attribute(
            ".cln.ListpeersPeersChannelsHtlcs",
            "#[derive(serde::Serialize)]",
        )
        .type_attribute(
            ".cln.ListpeersPeersChannelsHtlcsDirection",
            "#[derive(serde::Serialize)]",
        )
        .type_attribute(".cln.ListfundsResponse", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.ListfundsOutputs", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.ListfundsChannels", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.ListfundsOutputsStatus", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.NewaddrResponse", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.ChannelState", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.ConnectDirection", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.ConnectResponse", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.ConnectAddress", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.ConnectAddressType", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.KeysendResponse", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.KeysendStatus", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.FundchannelResponse", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.PayResponse", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.PayStatus", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.CreateinvoiceResponse", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.CreateinvoiceStatus", "#[derive(serde::Serialize)]")
        .type_attribute(".cln.InvoiceResponse", "#[derive(serde::Serialize)]")
        .compile(&["proto/node.proto"], &["proto"])
        .unwrap();
}
