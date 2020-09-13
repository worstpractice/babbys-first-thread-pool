pub type Closure = dyn FnOnce() + Send + 'static;

pub type Job = Box<Closure>;
