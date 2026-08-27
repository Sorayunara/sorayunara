# Security Policy

## Supported Versions

The Sorayunara core team actively maintains and releases security patches for the following versions:

| Version | Supported          | Security Fixes |
| :---    | :---:              | :---:          |
| 0.2.x   | :white_check_mark: | Full Active Support |
| 0.1.x   | :x:                | Deprecated (Upgrade to 0.2.x) |

---

## Reporting a Vulnerability

We take the security of the Sorayunara compiler, standard library, runtime, and package registry seriously. If you discover a security vulnerability, please report it responsibly:

### 1. Private Disclosure
**DO NOT** file a public GitHub issue for security vulnerabilities. Instead:
- Email our security team directly at: **security@sorayunara.org**
- Or use GitHub Private Vulnerability Reporting via [Security Advisories](https://github.com/Sorayunara/sorayunara/security/advisories/new).

### 2. What to Include in Your Report
To help us triage and resolve the issue quickly, please provide:
- A description of the vulnerability and its potential impact.
- A minimal reproducible example (`.sora` code snippet or CLI invocation).
- Affected version(s) and operating system (Linux, macOS, Windows).
- Any proposed mitigations or proof-of-concept exploits.

### 3. Response & Disclosure Process
- **Acknowledgement**: We will acknowledge receipt of your report within **48 hours**.
- **Assessment**: We will verify the vulnerability and determine the severity rating (CVSS).
- **Patch & Advisory**: A security patch will be developed and released alongside a CVE identifier and security advisory crediting the reporter.
- **Embargo**: We request that you keep details confidential until an official patch has been published.
