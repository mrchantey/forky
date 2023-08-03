# Forky CLI

> *Very early stage warning:*
> - breaking changes on patch versions
> - continued development not guaranteed
> - bugs ahoy

Various utilities to ease some of the repetitious work that comes with managing crates.

```auto_mod```

Its current incarnation is zero config and opinionated so you may want to play around with it on an empty project before integrating with existing codebases.

## Rules

- All files are modules *unless*
	- They start with an underscore
	- Their parent contains a double underscore

ie
```
test_dir
	__test_sub_dur
		_test_use.rs
		test_mod.rs
	_test_use.rs
	test_mod.rs
```
```rs
//test_dir.rs


```