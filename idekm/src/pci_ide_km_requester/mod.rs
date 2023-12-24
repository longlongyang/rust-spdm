// Copyright (c) 2023 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0 or MIT

#[derive(Debug, Default, Clone)]
pub struct IdekmReqContext {
    pub vendor_defined_req_payload_struct: spdmlib::message::VendorDefinedReqPayloadStruct,
    pub vendor_defined_rsp_payload_struct: spdmlib::message::VendorDefinedRspPayloadStruct,
}

pub mod pci_ide_km_req_query;
pub use pci_ide_km_req_query::*;

pub mod pci_ide_km_req_key_prog;
pub use pci_ide_km_req_key_prog::*;

pub mod pci_ide_km_req_key_set_go;
pub use pci_ide_km_req_key_set_go::*;

pub mod pci_ide_km_req_key_set_stop;
pub use pci_ide_km_req_key_set_stop::*;
