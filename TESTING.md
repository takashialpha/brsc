```
### TESTING.md Content

# Running Tests

You need to read this. You can't run tests without these steps.  
The code is prepared for release builds, but if you want to run tests to collaborate, follow these steps:

- Uncomment these lines in `main.rs`:

  ```rust
  #[cfg(test)]
  mod tests;
  ```

---

- Go to `cli.rs` and mark these as `pub`:

  ```rust
  pub struct Cli {
      pub expr: Option<String>,
   // +++

      pub expr: Option<String>,
   // +++

      pub quiet: bool,
   // +++
  }
  ```

- Go to `calculator.rs` and mark this as `pub`:

  ```rust
  pub struct Calculator {
      pub quiet: bool,
   // +++
  }
  ```

After reading this, if you are in the TEST.sh, exit to do the changes (ctrl+c or wait some secs to say no).
If you already did the changes, just wait some seconds.
```
