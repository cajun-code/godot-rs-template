use godot::prelude::*;
use godot::engine::{{parent_name}};
use godot::engine::{{parent_name}}Virtual;

#[derive(Debug, GodotClass)]
#[class(base={{parent_name}})]
struct {{class_name}}{

    #[base]
    body: Base<{{parent_name}}>,
}

#[godot_api]
impl {{parent_name}}Virtual for Rocket{
    fn init(base: Base<{{parent_name}}>)->Self{
        Self { body: base }
    }
    
}