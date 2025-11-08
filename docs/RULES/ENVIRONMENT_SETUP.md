# Environment Setup Rules

## Automatic .env File Loading

**RULE: Always load .env file in all Rust examples and scripts**

Every Rust example and script must include `dotenv::dotenv().ok();` at the beginning of the main() function to automatically load environment variables from the .env file.

### Implementation

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Rest of the code...
}
```

### Why This Rule Exists

1. **Consistency**: All examples should work out-of-the-box with existing .env files
2. **User Experience**: Users shouldn't need to manually export environment variables
3. **Development Efficiency**: Developers can run examples immediately without setup
4. **Error Prevention**: Eliminates "EnvVar(NotPresent)" errors when .env file exists

### Environment Variable Compatibility

The codebase supports both naming conventions:
- `SUPABASE_ANON_KEY` (legacy)
- `SUPABASE_API_KEY` (current)

Shell scripts should handle both for backward compatibility.

### Required Environment Variables

All examples need these environment variables in the .env file:
```
SUPABASE_URL=your_supabase_url
SUPABASE_API_KEY=your_supabase_api_key
FMP_API_KEY=your_fmp_api_key
ALPHA_VANTAGE_API_KEY=your_alpha_vantage_api_key
```

### Error Handling

If environment variables are missing, provide clear error messages:
```rust
if let Err(e) = std::env::var("SUPABASE_URL") {
    eprintln!("Error: SUPABASE_URL environment variable not set");
    eprintln!("Please add it to your .env file");
    std::process::exit(1);
}
```

This rule ensures all code works seamlessly with the existing .env file setup.

