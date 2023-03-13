use std::collections::HashMap;

use crate::compiler::CompileException;

// Represents a fire resource location such as myGame::Struct::Func
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct FireLocation {
    path: Vec<String>
}
impl FireLocation {
    pub fn new() -> FireLocation {
        FireLocation { path: Vec::with_capacity(16) }
    }
    pub fn with(&self, s: String) -> FireLocation {
        let mut path = self.path.clone();
        path.push(s);
        FireLocation { path }
    }
    pub fn parent(&self) -> FireLocation {
        let mut path = self.path.clone();
        path.pop();
        FireLocation { path }
    }
    pub fn push(&mut self, s: String) {
        self.path.push(s);
    }
    pub fn pop(&mut self) {
        self.path.pop();
    }
}

// Stores all relevant data about a project. Mainly a map of all resources.
pub struct StructuredFireProject {
    resources: HashMap<FireLocation, UncompiledFireResource>
}
impl StructuredFireProject {
    fn new() -> StructuredFireProject {
        StructuredFireProject { resources: HashMap::new() }
    }
    // Returns a resource at the location
    fn get_resource(&self, loc: &FireLocation) -> Option<&UncompiledFireResource> {
        self.resources.get(loc)
    }
    // Sets a resource at the location
    fn add_resource<'a>(&mut self, loc: &'a FireLocation, f: UncompiledFireResource) -> Result<(), CompileException<'a>> {
        if self.resources.contains_key(loc) {
            Err(CompileException::DuplicateResource(loc))
        } else {
            self.resources.insert(loc.clone(), f);
            Ok(())
        }
    }
}

// Represents an uncompiled Fire Resource. that is: "fn hello(?): ?". We only know the names.
// We cannot parse out any parameters or code about anything until we've structured the whole program to know all the names.
pub struct UncompiledFireResource {
    loc: FireLocation,
    resource_type: UncompiledResourceType
}
pub enum UncompiledResourceType {
    Function, // fn myFunc(?): ? { ? } 
    Method, // Ditto but inside of a struct
    AbstractMethod, // Ditto but inside of a trait, no code.
    Process, // pc myProcess(?) { ? }
    Struct, // struct MyStruct : ? { ?... funcs }
    Trait, // trait MyTrait : ? { absfuncs }
    Enum, // enum MyEnum : ? { vals = ? }
    Var, // let myVar = ?; (top-level)
    EnumConst // The vals inside of an enum
}