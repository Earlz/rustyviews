rustyviews
==========

a port of [EView](https://github.com/Earlz/EView) to rust

## Rationale

One thing that's always bugged me about view engines in strongly-typed languages is that they never make use of code generation to maintain the strong-typing features of the language. Instead of properties and classes, they use a dictionary/hashtable using strings as keys and untyped objects as values.

I've already done this with C#/.NET with EView, but T4 is painful to work into the typical .NET workflow. Rust has a community that really appreciates things being as strongly typed as possible, and with their tools, code generation has first class support(ish) in Cargo with build.rs scripts. So, I hope this idea is appreciated much more with the rust community :) 

## Examples/Expectations:

Example view:

	Templates/SomeView.rvhtml:
	{@ title as &Str; @}
	<html><head>
	<title>{=Title=}</title>
	<body>
	Hey check out the dynamic title
	</body>
	</html>

Usage:

	let template=SomeView::new();
	template.title="Foo bar";
	println("{}", template.render()); //could also render to a stream probably as well

Output:

	<html><head>
	<title>Foo bar</title>
	<body>
	Hey check out the dynamic title
	</body>
	</html>

