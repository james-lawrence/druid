use super::displays;

pub type Option = fn(&Config) -> Config;

fn new (c: Config) -> Manager {
    return 
}

// auto blocking eventloop.
fn auto(c: Config) -> std::Result<()> {
    let mut eventloop = calloop::EventLoop::try_new()?;
}

pub struct Builder {

}

impl Builder {
    fn manager() -> impl displays::Manager {
        
    }
}

pub struct Manager {

}

impl Manager {

}

impl displays::Manager for Manager {

}