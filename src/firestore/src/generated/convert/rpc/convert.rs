// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Code generated by sidekick. DO NOT EDIT.

impl gaxi::prost::Convert<ErrorInfo> for rpc::model::ErrorInfo {
    fn cnv(self) -> ErrorInfo {
        ErrorInfo {
            reason: self.reason.cnv(),
            domain: self.domain.cnv(),
            metadata: self.metadata.into_iter().map(|(k, v)| (k.cnv(), v.cnv())).collect(),
        }
    }
}

impl gaxi::prost::Convert<rpc::model::ErrorInfo> for ErrorInfo {
    fn cnv(self) -> rpc::model::ErrorInfo {
        rpc::model::ErrorInfo::new()
            .set_reason(self.reason)
            .set_domain(self.domain)
            .set_metadata(self.metadata.into_iter().map(|(k, v)| (k.cnv(), v.cnv())))
    }
}

impl gaxi::prost::Convert<RetryInfo> for rpc::model::RetryInfo {
    fn cnv(self) -> RetryInfo {
        RetryInfo {
            retry_delay: self.retry_delay.map(|v| v.cnv()),
        }
    }
}

impl gaxi::prost::Convert<rpc::model::RetryInfo> for RetryInfo {
    fn cnv(self) -> rpc::model::RetryInfo {
        rpc::model::RetryInfo::new()
            .set_retry_delay(self.retry_delay.map(|v| v.cnv()))
    }
}

impl gaxi::prost::Convert<DebugInfo> for rpc::model::DebugInfo {
    fn cnv(self) -> DebugInfo {
        DebugInfo {
            detail: self.detail.cnv(),
            stack_entries: self.stack_entries.into_iter().map(|v| v.cnv()).collect(),
        }
    }
}

impl gaxi::prost::Convert<rpc::model::DebugInfo> for DebugInfo {
    fn cnv(self) -> rpc::model::DebugInfo {
        rpc::model::DebugInfo::new()
            .set_detail(self.detail)
            .set_stack_entries(self.stack_entries.into_iter().map(|v| v.cnv()))
    }
}

impl gaxi::prost::Convert<quota_failure::Violation> for rpc::model::quota_failure::Violation {
    fn cnv(self) -> quota_failure::Violation {
        quota_failure::Violation {
            subject: self.subject.cnv(),
            description: self.description.cnv(),
            api_service: self.api_service.cnv(),
            quota_metric: self.quota_metric.cnv(),
            quota_id: self.quota_id.cnv(),
            quota_value: self.quota_value.cnv(),
            future_quota_value: self.future_quota_value.map(|v| v.cnv()),
            quota_dimensions: self.quota_dimensions.into_iter().map(|(k, v)| (k.cnv(), v.cnv())).collect(),
        }
    }
}

impl gaxi::prost::Convert<rpc::model::quota_failure::Violation> for quota_failure::Violation {
    fn cnv(self) -> rpc::model::quota_failure::Violation {
        rpc::model::quota_failure::Violation::new()
            .set_subject(self.subject)
            .set_description(self.description)
            .set_api_service(self.api_service)
            .set_quota_metric(self.quota_metric)
            .set_quota_id(self.quota_id)
            .set_quota_value(self.quota_value)
            .set_future_quota_value(self.future_quota_value.map(|v| v.cnv()))
            .set_quota_dimensions(self.quota_dimensions.into_iter().map(|(k, v)| (k.cnv(), v.cnv())))
    }
}

impl gaxi::prost::Convert<QuotaFailure> for rpc::model::QuotaFailure {
    fn cnv(self) -> QuotaFailure {
        QuotaFailure {
            violations: self.violations.into_iter().map(|v| v.cnv()).collect(),
        }
    }
}

impl gaxi::prost::Convert<rpc::model::QuotaFailure> for QuotaFailure {
    fn cnv(self) -> rpc::model::QuotaFailure {
        rpc::model::QuotaFailure::new()
            .set_violations(self.violations.into_iter().map(|v| v.cnv()))
    }
}

impl gaxi::prost::Convert<precondition_failure::Violation> for rpc::model::precondition_failure::Violation {
    fn cnv(self) -> precondition_failure::Violation {
        precondition_failure::Violation {
            r#type: self.r#type.cnv(),
            subject: self.subject.cnv(),
            description: self.description.cnv(),
        }
    }
}

impl gaxi::prost::Convert<rpc::model::precondition_failure::Violation> for precondition_failure::Violation {
    fn cnv(self) -> rpc::model::precondition_failure::Violation {
        rpc::model::precondition_failure::Violation::new()
            .set_type(self.r#type)
            .set_subject(self.subject)
            .set_description(self.description)
    }
}

impl gaxi::prost::Convert<PreconditionFailure> for rpc::model::PreconditionFailure {
    fn cnv(self) -> PreconditionFailure {
        PreconditionFailure {
            violations: self.violations.into_iter().map(|v| v.cnv()).collect(),
        }
    }
}

impl gaxi::prost::Convert<rpc::model::PreconditionFailure> for PreconditionFailure {
    fn cnv(self) -> rpc::model::PreconditionFailure {
        rpc::model::PreconditionFailure::new()
            .set_violations(self.violations.into_iter().map(|v| v.cnv()))
    }
}

impl gaxi::prost::Convert<bad_request::FieldViolation> for rpc::model::bad_request::FieldViolation {
    fn cnv(self) -> bad_request::FieldViolation {
        bad_request::FieldViolation {
            field: self.field.cnv(),
            description: self.description.cnv(),
            reason: self.reason.cnv(),
            localized_message: self.localized_message.map(|v| v.cnv()),
        }
    }
}

impl gaxi::prost::Convert<rpc::model::bad_request::FieldViolation> for bad_request::FieldViolation {
    fn cnv(self) -> rpc::model::bad_request::FieldViolation {
        rpc::model::bad_request::FieldViolation::new()
            .set_field(self.field)
            .set_description(self.description)
            .set_reason(self.reason)
            .set_localized_message(self.localized_message.map(|v| v.cnv()))
    }
}

impl gaxi::prost::Convert<BadRequest> for rpc::model::BadRequest {
    fn cnv(self) -> BadRequest {
        BadRequest {
            field_violations: self.field_violations.into_iter().map(|v| v.cnv()).collect(),
        }
    }
}

impl gaxi::prost::Convert<rpc::model::BadRequest> for BadRequest {
    fn cnv(self) -> rpc::model::BadRequest {
        rpc::model::BadRequest::new()
            .set_field_violations(self.field_violations.into_iter().map(|v| v.cnv()))
    }
}

impl gaxi::prost::Convert<RequestInfo> for rpc::model::RequestInfo {
    fn cnv(self) -> RequestInfo {
        RequestInfo {
            request_id: self.request_id.cnv(),
            serving_data: self.serving_data.cnv(),
        }
    }
}

impl gaxi::prost::Convert<rpc::model::RequestInfo> for RequestInfo {
    fn cnv(self) -> rpc::model::RequestInfo {
        rpc::model::RequestInfo::new()
            .set_request_id(self.request_id)
            .set_serving_data(self.serving_data)
    }
}

impl gaxi::prost::Convert<ResourceInfo> for rpc::model::ResourceInfo {
    fn cnv(self) -> ResourceInfo {
        ResourceInfo {
            resource_type: self.resource_type.cnv(),
            resource_name: self.resource_name.cnv(),
            owner: self.owner.cnv(),
            description: self.description.cnv(),
        }
    }
}

impl gaxi::prost::Convert<rpc::model::ResourceInfo> for ResourceInfo {
    fn cnv(self) -> rpc::model::ResourceInfo {
        rpc::model::ResourceInfo::new()
            .set_resource_type(self.resource_type)
            .set_resource_name(self.resource_name)
            .set_owner(self.owner)
            .set_description(self.description)
    }
}

impl gaxi::prost::Convert<help::Link> for rpc::model::help::Link {
    fn cnv(self) -> help::Link {
        help::Link {
            description: self.description.cnv(),
            url: self.url.cnv(),
        }
    }
}

impl gaxi::prost::Convert<rpc::model::help::Link> for help::Link {
    fn cnv(self) -> rpc::model::help::Link {
        rpc::model::help::Link::new()
            .set_description(self.description)
            .set_url(self.url)
    }
}

impl gaxi::prost::Convert<Help> for rpc::model::Help {
    fn cnv(self) -> Help {
        Help {
            links: self.links.into_iter().map(|v| v.cnv()).collect(),
        }
    }
}

impl gaxi::prost::Convert<rpc::model::Help> for Help {
    fn cnv(self) -> rpc::model::Help {
        rpc::model::Help::new()
            .set_links(self.links.into_iter().map(|v| v.cnv()))
    }
}

impl gaxi::prost::Convert<LocalizedMessage> for rpc::model::LocalizedMessage {
    fn cnv(self) -> LocalizedMessage {
        LocalizedMessage {
            locale: self.locale.cnv(),
            message: self.message.cnv(),
        }
    }
}

impl gaxi::prost::Convert<rpc::model::LocalizedMessage> for LocalizedMessage {
    fn cnv(self) -> rpc::model::LocalizedMessage {
        rpc::model::LocalizedMessage::new()
            .set_locale(self.locale)
            .set_message(self.message)
    }
}

impl gaxi::prost::Convert<HttpRequest> for rpc::model::HttpRequest {
    fn cnv(self) -> HttpRequest {
        HttpRequest {
            method: self.method.cnv(),
            uri: self.uri.cnv(),
            body: self.body.cnv(),
            headers: self.headers.into_iter().map(|v| v.cnv()).collect(),
        }
    }
}

impl gaxi::prost::Convert<rpc::model::HttpRequest> for HttpRequest {
    fn cnv(self) -> rpc::model::HttpRequest {
        rpc::model::HttpRequest::new()
            .set_method(self.method)
            .set_uri(self.uri)
            .set_body(self.body)
            .set_headers(self.headers.into_iter().map(|v| v.cnv()))
    }
}

impl gaxi::prost::Convert<HttpResponse> for rpc::model::HttpResponse {
    fn cnv(self) -> HttpResponse {
        HttpResponse {
            status: self.status.cnv(),
            reason: self.reason.cnv(),
            body: self.body.cnv(),
            headers: self.headers.into_iter().map(|v| v.cnv()).collect(),
        }
    }
}

impl gaxi::prost::Convert<rpc::model::HttpResponse> for HttpResponse {
    fn cnv(self) -> rpc::model::HttpResponse {
        rpc::model::HttpResponse::new()
            .set_status(self.status)
            .set_reason(self.reason)
            .set_body(self.body)
            .set_headers(self.headers.into_iter().map(|v| v.cnv()))
    }
}

impl gaxi::prost::Convert<HttpHeader> for rpc::model::HttpHeader {
    fn cnv(self) -> HttpHeader {
        HttpHeader {
            key: self.key.cnv(),
            value: self.value.cnv(),
        }
    }
}

impl gaxi::prost::Convert<rpc::model::HttpHeader> for HttpHeader {
    fn cnv(self) -> rpc::model::HttpHeader {
        rpc::model::HttpHeader::new()
            .set_key(self.key)
            .set_value(self.value)
    }
}
