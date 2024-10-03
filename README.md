# Build Generator
## Why?
The purpose of this repo is to provide a command line utility built in rust for generating common project types that I use for different languages.

Is it necessary? No. 

Is it an interesting yak-shaving project? Good enough.
# How?
- [ ] Project Types
	- [ ] C++/C (Build2)
	- [x] Rust (Cargo)
	- [ ] Python (PipEnv controlled virtual environment)
- [ ] Support templates
	- [ ] Empty targets
		- [ ] Build2
		- [x] Cargo
		- [ ] Python (via virtual env)
	- [ ] Binary targets
		- [ ] Single executables
			- [ ] Build2
			- [x] Cargo
		- [ ] Python
	- [ ] Library only targets
		- [ ] Single library
			- [ ] Build2
			- [x] Cargo
		- [ ] Python (Single/Multi file)
	- [ ] Nested targets
		- [ ] Nested libraries
			- [ ] Build2
			- [x] Cargo
		- Nested libraries inside of a build root in build2, cargo
		- Multiple modules inside a mod root
	- [ ] Nested targets w/ binary target
		- Binary and lib-root
			- [ ] Build2
			- [x] Cargo
		- [ ] Main file with library module
			- [ ] Python
- [ ] Clone projects
	- [ ] Default minimal clone with custom name
# Resources
