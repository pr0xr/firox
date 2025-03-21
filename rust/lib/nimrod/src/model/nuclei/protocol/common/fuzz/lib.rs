use crate::model::nuclei::protocol::lib::ExecutorOptions;


// Rule is a single rule which describes how to fuzz the request
pub struct Rule {
	// description: |
	//   Type is the type of fuzzing rule to perform.
	//
	//   replace replaces the values entirely. prefix prefixes the value. postfix postfixes the value
	//   and infix places between the values.
	// values:
	//   - "replace"
	//   - "prefix"
	//   - "postfix"
	//   - "infix"
	pub r#type: String, // `yaml:"type,omitempty" json:"type,omitempty" jsonschema:"title=type of rule,description=Type of fuzzing rule to perform,enum=replace,enum=prefix,enum=postfix,enum=infix"`
	//ruleType ruleType
	// description: |
	//   Part is the part of request to fuzz.
	//
	//   query fuzzes the query part of url. More parts will be added later.
	// values:
	//   - "query"
	pub part: String, // `yaml:"part,omitempty" json:"part,omitempty" jsonschema:"title=part of rule,description=Part of request rule to fuzz,enum=query"`
	//partType partType
	// description: |
	//   Mode is the mode of fuzzing to perform.
	//
	//   single fuzzes one value at a time. multiple fuzzes all values at same time.
	// values:
	//   - "single"
	//   - "multiple"
	pub mode: String, // `yaml:"mode,omitempty" json:"mode,omitempty" jsonschema:"title=mode of rule,description=Mode of request rule to fuzz,enum=single,enum=multiple"`
	//modeType modeType

	// description: |
	//   Keys is the optional list of key named parameters to fuzz.
	// examples:
	//   - name: Examples of keys
	//     value: >
	//       []string{"url", "file", "host"}
	pub keys: Vec<String>, // `yaml:"keys,omitempty" json:"keys,omitempty" jsonschema:"title=keys of parameters to fuzz,description=Keys of parameters to fuzz"`
	//keysMap map[string]struct{}
	// description: |
	//   KeysRegex is the optional list of regex key parameters to fuzz.
	// examples:
	//   - name: Examples of key regex
	//     value: >
	//       []string{"url.*"}
	pub keys_regex: Vec<String>, // `yaml:"keys-regex,omitempty" json:"keys-regex,omitempty" jsonschema:"title=keys regex to fuzz,description=Regex of parameter keys to fuzz"`
	//keysRegex []*regexp.Regexp
	// description: |
	//   Values is the optional list of regex value parameters to fuzz.
	// examples:
	//   - name: Examples of value regex
	//     value: >
	//       []string{"https?://.*"}
	pub values_regex: Vec<String>, //yaml:"values,omitempty" json:"values,omitempty" jsonschema:"title=values regex to fuzz,description=Regex of parameter values to fuzz"`
	//valuesRegex []*regexp.Regexp

	// description: |
	//   Fuzz is the list of payloads to perform substitutions with.
	// examples:
	//   - name: Examples of fuzz
	//     value: >
	//       []string{"{{ssrf}}", "{{interactsh-url}}", "example-value"}
	pub fuzz: String, // `yaml:"fuzz,omitempty" json:"fuzz,omitempty" jsonschema:"title=payloads of fuzz rule,description=Payloads to perform fuzzing substitutions with"`

	pub options: ExecutorOptions,
	pub generator: PayloadGenerator,
}
