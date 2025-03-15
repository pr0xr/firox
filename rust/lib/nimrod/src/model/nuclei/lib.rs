use serde_json::value::Value;
use crate::model::nuclei::protocol::lib::Request;


#[derive(Default)]
pub struct Template {
    pub id: String,
    pub info: Info,
    pub flow: String,
    // DEPRECATED
    // type is request
    pub requests_http: String,
    // pub requestsWithHttp
    pub requests_dns: String,
    pub requests_file: String,
    //pub requests_network
    pub requests_with_tcp: String,
    pub requests_headless: String,
    pub requests_ssl: String,
    pub requests_websocket: String,
    pub requests_whois: String,
    pub requests_code: String,
    pub requests_javascript: String,
    pub workflow: Workflow,
    // CompiledWorkflow
    pub self_contained: bool,
    pub stop_at_first_match: bool,
    //pub signature:
    //pub variables:
    pub constants: Value,//map[string]interface{}
    pub total_requests: i32,
    //pub executor: Executor,
    pub path: String,
    pub verified: bool,
    // protocol.Request
    //pub request_queue: Vec<dyn Request>,
    pub request_queue: String,
    pub imported_files: Vec<String>,
}


#[derive(Default)]
pub struct Info {
    pub name: String,
    pub authors: String,
    pub tags: String,
    pub description: String,
    pub impact: String,
    pub reference: String,
    pub severity_holder: String,
    pub metadata: String,
    pub classification: String,
    pub remediation: String,
}

#[derive(Default)]
pub struct Workflow {}

#[derive(Default)]
pub struct Classification {
    pub cveid: String,
    pub cweid: String,
    pub cvssmetrics: String,
    pub cvssscore: f64,
    pub epssscore: f64,
    pub epsspercentile: f64,
    pub cpe: String,
}
