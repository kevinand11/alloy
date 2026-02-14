pub type ScopeId = usize;
pub type ScopeTypeId = usize;
pub type ScopeVarId = usize;

pub const UNIT_TYPE_ID: ScopeTypeId = 1;
pub const INT_TYPE_ID: ScopeTypeId = 2;
pub const FLOAT_TYPE_ID: ScopeTypeId = 3;
pub const BOOL_TYPE_ID: ScopeTypeId = 4;

pub struct Scope {
    pub id: ScopeId,
    pub parent: Option<ScopeId>,
    pub types: Vec<ScopedType>,
    pub vars: Vec<ScopedVar>,
}

impl Scope {
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
    pub id: ScopeTypeId,
    pub name: String,
}

pub struct ScopedVar {
    pub id: ScopeVarId,
    pub name: String,
    pub type_id: ScopeTypeId,
    pub mutable: bool,
}

pub struct ScopeManager {
    next_type_id: ScopeTypeId,
    next_var_id: ScopeVarId,
    scopes: Vec<Scope>,
    pub cur: ScopeId,
}

impl ScopeManager {
    pub fn new() -> Self {
        let global_scope = Scope {
            id: 0,
            parent: None,
            types: vec![
                ScopedType {
                    id: UNIT_TYPE_ID,
                    name: "Unit".to_string(),
                },
                ScopedType {
                    id: INT_TYPE_ID,
                    name: "Int".to_string(),
                },
                ScopedType {
                    id: FLOAT_TYPE_ID,
                    name: "Float".to_string(),
                },
                ScopedType {
                    id: BOOL_TYPE_ID,
                    name: "Bool".to_string(),
                },
            ],
            vars: vec![],
        };
        Self {
            next_type_id: global_scope.types.len() + 1,
            next_var_id: 0,
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

    pub fn lookup_type_by_name(&self, ty_name: &str, scope_id: ScopeId) -> Option<&ScopedType> {
        let scope = &self.scopes[scope_id];
        if let Some(ty) = scope.types.iter().find(|t| t.name == ty_name) {
            Some(ty)
        } else if let Some(parent_id) = scope.parent {
            self.lookup_type_by_name(ty_name, parent_id)
        } else {
            None
        }
    }

    pub fn lookup_type(&self, ty_id: ScopeTypeId, scope_id: ScopeId) -> Option<&ScopedType> {
        let scope = &self.scopes[scope_id];
        if let Some(ty) = scope.types.iter().find(|t| t.id == ty_id) {
            Some(ty)
        } else if let Some(parent_id) = scope.parent {
            self.lookup_type(ty_id, parent_id)
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

    pub fn add_var(&mut self, var_name: &str, ty_id: ScopeTypeId, mutable: bool) {
        let scope = match self.scopes.get_mut(self.cur) {
            Some(scope) => scope,
            None => panic!("Current scope does not exist"),
        };
        let var = ScopedVar {
            id: self.next_var_id,
            name: var_name.to_string(),
            type_id: ty_id,
            mutable,
        };
        scope.vars.push(var);
        self.next_var_id += 1;
    }

    pub fn add_type(&mut self, ty_name: &str) {
        let scope = match self.scopes.get_mut(self.cur) {
            Some(scope) => scope,
            None => panic!("Current scope does not exist"),
        };
        let ty = ScopedType {
            id: self.next_type_id,
            name: ty_name.to_string(),
        };
        scope.types.push(ty);
        self.next_type_id += 1;
    }
}
