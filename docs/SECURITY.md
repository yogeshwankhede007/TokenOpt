# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Reporting a Vulnerability

We take security seriously. If you discover a security vulnerability, please report it responsibly.

### Private Disclosure

**Email**: security@tokenopt.dev

Please include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

### Response Timeline

- **Acknowledgment**: Within 24 hours
- **Initial Assessment**: Within 72 hours
- **Fix Development**: Within 7-30 days (depending on severity)
- **Public Disclosure**: After fix is released

### Severity Classification

| Severity | CVSS Range | Response Time |
|----------|------------|---------------|
| Critical | 9.0-10.0   | 24 hours      |
| High     | 7.0-8.9    | 72 hours      |
| Medium   | 4.0-6.9    | 7 days        |
| Low      | 0.1-3.9    | 30 days       |

## Security Measures

### Code Security
- No `unsafe` code in core library
- All dependencies audited via `cargo audit`
- License compliance via `cargo deny`
- SAST scanning in CI/CD
- Dependency update automation

### Supply Chain
- Locked dependencies (Cargo.lock)
- Signed releases
- Reproducible builds
- SBOM generation

### Data Protection
- No persistent storage by default
- Optional encrypted disk cache
- No telemetry without opt-in
- PII detection (optional)

## Vulnerability History

| Version | CVE | Severity | Fixed |
|---------|-----|----------|-------|
| -       | -   | -        | -     |

## Bug Bounty

We offer rewards for verified security vulnerabilities:

| Severity | Reward |
|----------|--------|
| Critical | $5,000 |
| High     | $2,500 |
| Medium   | $1,000 |
| Low      | $500   |

Submit via security@tokenopt.dev with "Bug Bounty" in subject.