pub type ScopeId = usize;

pub struct Scope {
    pub id: ScopeId,
    pub parent: Option<ScopeId>,
    pub types: Vec<ScopedType>,
    pub vars: Vec<ScopedVar>,
}

impl Scope {
    pub fn global() -> Self {
        Scope {
            id: 0,
            parent: None,
            types: vec![
                ScopedType {
                    name: "Unit".to_string(),
                },
                ScopedType {
                    name: "Int".to_string(),
                },
                ScopedType {
                    name: "Float".to_string(),
                },
                ScopedType {
                    name: "Bool".to_string(),
                },
            ],
            vars: vec![],
        }
    }

    pub fn child(id: ScopeId, parent: ScopeId) -> Self {
        Scope {
            id,
            parent: Some(parent),
            types: vec![],
            vars: vec![],
        }
    }
}

pub struct ScopedType {
    pub name: String,
}

pub struct ScopedVar {
    pub name: String,
    pub type_name: String,
}

pub struct ScopeManager {
    scopes: Vec<Scope>,
    pub cur: ScopeId,
}

impl ScopeManager {
    pub fn new() -> Self {
        let global_scope = Scope::global();
        Self {
            cur: global_scope.id,
            scopes: vec![global_scope],
        }
    }

    pub fn create_scope(&mut self, parent: ScopeId) -> ScopeId {
        let id = self.scopes.len();
        let scope = Scope::child(id, parent);
        self.scopes.push(scope);
        id
    }

    pub fn lookup_type(&self, ty_name: &str, scope_id: ScopeId) -> Option<&ScopedType> {
        let scope = &self.scopes[scope_id];
        if let Some(ty) = scope.types.iter().find(|t| t.name == ty_name) {
            Some(ty)
        } else if let Some(parent_id) = scope.parent {
            self.lookup_type(ty_name, parent_id)
        } else {
            None
        }
    }

    pub fn lookup_var(&self, var_name: &str, scope_id: ScopeId) -> Option<&ScopedVar> {
        let scope = &self.scopes[scope_id];
        if let Some(var) = scope.vars.iter().find(|v| v.name == var_name) {
            Some(var)
        } else if let Some(parent_id) = scope.parent {
            self.lookup_var(var_name, parent_id)
        } else {
            None
        }
    }

    pub fn add_var(&mut self, var_name: &str, ty_name: &str) {
        let scope = match self.scopes.get_mut(self.cur) {
            Some(scope) => scope,
            None => panic!("Current scope does not exist"),
        };
        let var = ScopedVar {
            name: var_name.to_string(),
            type_name: ty_name.to_string(),
        };
        scope.vars.push(var);
    }

    pub fn add_type(&mut self, ty_name: &str) {
        let scope = match self.scopes.get_mut(self.cur) {
            Some(scope) => scope,
            None => panic!("Current scope does not exist"),
        };
        let ty = ScopedType {
            name: ty_name.to_string(),
        };
        scope.types.push(ty);
    }
}
