// Copyright (c) 2023 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0 or MIT

use codec::Codec;
use codec::Writer;
use spdmlib::error::SPDM_STATUS_BUFFER_FULL;
use spdmlib::error::SPDM_STATUS_INVALID_MSG_FIELD;
use spdmlib::{error::SpdmResult, requester::RequesterContext};

use crate::pci_tdisp::vendor_id;
use crate::pci_tdisp::InterfaceId;
use crate::pci_tdisp::ReqStopInterfaceRequest;
use crate::pci_tdisp::RspStopInterfaceResponse;
use crate::pci_tdisp::TdispMessageHeader;
use crate::pci_tdisp::TdispRequestResponseCode;
use crate::pci_tdisp::TdispVersion;
use crate::pci_tdisp::STANDARD_ID;

use super::TdispReqContext;

impl TdispReqContext {
    #[maybe_async::maybe_async]
    pub async fn pci_tdisp_req_stop_interface_request(
        &mut self,
        // IN
        spdm_requester: &mut RequesterContext,
        session_id: u32,
        interface_id: InterfaceId,
    ) -> SpdmResult {
        let mut writer = Writer::init(
            &mut self
                .vendor_defined_req_payload_struct
                .vendor_defined_req_payload,
        );

        self.vendor_defined_req_payload_struct.req_length = ReqStopInterfaceRequest {
            message_header: TdispMessageHeader {
                interface_id,
                message_type: TdispRequestResponseCode::STOP_INTERFACE_REQUEST,
                tdisp_version: TdispVersion {
                    major_version: 1,
                    minor_version: 0,
                },
            },
        }
        .encode(&mut writer)
        .map_err(|_| SPDM_STATUS_BUFFER_FULL)?
            as u16;

        spdm_requester
            .send_spdm_vendor_defined_request(
                Some(session_id),
                STANDARD_ID,
                vendor_id(),
                &self.vendor_defined_req_payload_struct,
                &mut self.vendor_defined_rsp_payload_struct,
            )
            .await?;

        let rsp_stop_interface_response = RspStopInterfaceResponse::read_bytes(
            &self
                .vendor_defined_rsp_payload_struct
                .vendor_defined_rsp_payload
                [..self.vendor_defined_rsp_payload_struct.rsp_length as usize],
        )
        .ok_or(SPDM_STATUS_INVALID_MSG_FIELD)?;

        if rsp_stop_interface_response.message_header.tdisp_version
            != (TdispVersion {
                major_version: 1,
                minor_version: 0,
            })
            || rsp_stop_interface_response.message_header.message_type
                != TdispRequestResponseCode::STOP_INTERFACE_RESPONSE
            || rsp_stop_interface_response.message_header.interface_id != interface_id
        {
            return Err(SPDM_STATUS_INVALID_MSG_FIELD);
        }

        Ok(())
    }
}
