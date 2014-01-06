use method::Method;

pub struct Context
{
	name: ~str,
	methods: ~[Method],
	subContexts: ~[Context],
	action:	fn() -> ()
}
