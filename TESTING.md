### // this is TESTING.md content.
# Running tests
You need to read this. you will cant do tests without this steps.
The code is prepared for release builds. but if you want to run tests to colaborate, follow this steps:

- uncomment this lines in main.rs: 
'''
#[cfg(test)]
mod tests;
'''

---

- go to cli.rs and mark these as pub.

'''
pub struct Cli {
	pub expr: Option<String>,
	+++

	pub expr: Option<String>,
	+++

	pub quiet: bool,
   	+++
}
'''

- go to calculator.rs and mark this as pub.

'''
pub struct Calculator {
	pub quiet: bool,
	+++
}
'''
That's it. After doing this steps you can proceed. if you already read. wait some seconds.
