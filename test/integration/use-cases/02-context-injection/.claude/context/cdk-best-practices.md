# AWS CDK Best Practices

## Construct Organization

- Use L2 (high-level) constructs when available
- Create custom constructs for reusable patterns
- Keep stacks focused and single-purpose

## Naming Conventions

- Use PascalCase for construct IDs
- Use kebab-case for physical resource names
- Include environment prefix in resource names

## Security

- Never hardcode secrets - use AWS Secrets Manager
- Apply least privilege IAM policies
- Enable encryption at rest for all storage

## Testing

- Write unit tests for custom constructs
- Use CDK assertions for infrastructure testing
- Test synthesized templates with cfn-lint

---
*This context was injected by CCH to help with CDK development.*
