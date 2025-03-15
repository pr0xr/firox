use serde_json::value::Value;
use crate::model::nuclei::operators::matchers::matchers::Matcher;
use crate::model::nuclei::output::lib::ResultEvent;
use crate::model::nuclei::operators::lib::Operators;
use crate::model::nuclei::lib::Info;
use anyhow::Error;


pub trait Executor {
    fn compile();
    fn requests();
    fn execute();
    fn execute_with_results();
}

// ExecutorOptions contains the configuration options for executer clients
pub struct ExecutorOptions {
	// TemplateID is the ID of the template for the request
	pub template_id: String,
	// TemplatePath is the path of the template for the request
	pub template_path: String,
	// TemplateInfo contains information block of the template request
	pub template_info: Info,
	// Output is a writer interface for writing output events from executer.
	pub output: Writer,
	// Options contains configuration options for the executer.
	pub options: Options,
	// IssuesClient is a client for nuclei issue tracker reporting
	pub issues_client: ReportingClient,
	// Progress is a progress client for scan reporting
	pub progress: Progress,
	// RateLimiter is a rate-limiter for limiting sent number of requests.
	pub rate_limiter: RateLimit,
	// Catalog is a template catalog implementation for nuclei
	pub catalog:Catalog,
	// ProjectFile is the project file for nuclei
	pub project_file: ProjectFile,
	// Browser is a browser engine for running headless templates
	pub browser: Browser,
	// Interactsh is a client for interactsh oob polling server
	pub interactsh: InteractshClient,
	// HostErrorsCache is an optional cache for handling host errors
	pub host_errors_cache: CacheInterface,
	// Stop execution once first match is found (Assigned while parsing templates)
	// Note: this is different from Options.StopAtFirstMatch (Assigned from CLI option)
	pub stop_at_first_match: bool,
	// Variables is a list of variables from template
	pub variables: Variable,
	// Constants is a list of constants from template
	pub constants: Value,
	// ExcludeMatchers is the list of matchers to exclude
	pub exclude_matchers: ExcludeMatchers,
	// InputHelper is a helper for input normalization
	pub input_helper: Helper,

	pub operators: Vec<Operators>, // only used by offlinehttp module

	// DoNotCache bool disables optional caching of the templates structure
	pub do_not_cache: bool,

	pub colorizer: bool,
	pub workflow_loader:WorkflowLoader,
	pub resume_cfg: ResumeCfg,
	// ProtocolType is the type of the template
	pub protocol_type: ProtocolType,

	// Flow is execution flow for the template (written in javascript)
	pub flow: String,
	// IsMultiProtocol is true if template has more than one protocol
	pub is_multi_protocol: bool,
	// templateStore is a map which contains template context for each scan  (i.e input * template-id pair)
	//pub template_ctx_store:  *mapsutil.SyncLockMap[string, *contextargs.Context]
	// JsCompiler is abstracted javascript compiler which adds node modules and provides execution
	// environment for javascript templates
	pub jscompiler: JSCompiler,
}


// Request is an interface implemented any protocol based request generator.
pub trait Request {
	// Compile compiles the request generators preparing any requests possible.
	fn compile(options: ExecutorOptions) -> Error;
	// Requests returns the total number of requests the rule will perform
	fn requests() -> i32;
	// GetID returns the ID for the request if any. IDs are used for multi-request
	// condition matching. So, two requests can be sent and their match can
	// be evaluated from the third request by using the IDs for both requests.
	fn get_id() -> String;
	// Match performs matching operation for a matcher on model and returns:
	// true and a list of matched snippets if the matcher type is supports it
	// otherwise false and an empty string slice
	fn r#match(data: Value, matcher: Matcher) -> Result<bool, Vec<String>>;
	// Extract performs extracting operation for an extractor on model and returns true or false.
	fn extract(data: Value, matcher: Extractor) -> Value;
	// ExecuteWithResults executes the protocol requests and returns results instead of writing them.
	fn execute_with_results(input: Context) -> Error;
	// MakeResultEventItem creates a result event from internal wrapped event. Intended to be used by MakeResultEventItem internally
	fn make_result_event_item(wrapped: InternalWrappedEvent) -> ResultEvent;
	// MakeResultEvent creates a flat list of result events from an internal wrapped event, based on successful matchers and extracted data
    fn make_result_event(wrapped: InternalWrappedEvent) -> Vec<ResultEvent>;
	// GetCompiledOperators returns a list of the compiled operators
	fn get_compiled_operators() -> Vec<Operators>;
	// Type returns the type of the protocol request
	fn r#type() -> ProtocolType;
}
