use std::hashmap::HashMap;

use method::Method;

pub struct Context
{
	name: ~str,
	methods: ~[Method],
	subContextMap: HashMap<~str, Context>,
	action:	fn() -> ()
}
