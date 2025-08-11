# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- GitHub Actions CI/CD workflows
- Automated testing across multiple platforms
- Security audit checks
- Documentation deployment
- Code coverage reporting

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2] - 2025-08-11

### Added
- Complete CI/CD pipeline with GitHub Actions
- Multi-platform testing (Ubuntu, Windows, macOS)  
- Automated security auditing with cargo-audit
- Code coverage reporting with codecov
- Automated documentation generation and deployment
- Dependency monitoring workflow

### Fixed
- Clippy warnings and code formatting issues
- Deprecated actions-rs/audit replaced with direct cargo audit

## [0.1.1] - 2025-08-11

### Fixed
- Tool calling API examples in README.md
- Images API to use correct dall-e-3 model
- Documentation structure and clarity
- Code examples to match actual API implementation

### Changed
- Improved documentation structure

## [0.1.0] - 2025-08-11

### Added
- Initial release
- Chat Completions API support
- Embeddings API support  
- Images generation (DALL-E)
- Responses API with streaming support
- Tool calling functionality
- SSE streaming for real-time responses
- Comprehensive error handling
- Retry logic with exponential backoff
- File upload/download support
- Multiple authentication methods
- Builder pattern for client configuration
- Extensive examples and documentation

### Features
- **Chat Completions**: GPT-4, GPT-3.5-turbo with streaming
- **Embeddings**: Text embeddings generation
- **Images**: DALL-E image generation
- **Responses**: Beta responses API with advanced reasoning
- **Tool Calling**: Function calling with structured outputs
- **Streaming**: Real-time SSE streaming
- **Async/Await**: Built on tokio for high performance
- **Type Safety**: Comprehensive Rust types for all endpoints

[Unreleased]: https://github.com/neeboo/openai-sdk-rs/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/neeboo/openai-sdk-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/neeboo/openai-sdk-rs/releases/tag/v0.1.0
