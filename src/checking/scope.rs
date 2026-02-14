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
}

impl Scope {
    pub fn child(id: ScopeId, parent: ScopeId) -> Self {
        Scope {
            id,
            parent: Some(parent),
        }
    }
}

pub struct ScopedType {
    pub id: ScopeTypeId,
    pub parent_id: Option<ScopeTypeId>,
    pub name: String,
    pub scope_id: ScopeId,
}

pub struct ScopedVar {
    pub id: ScopeVarId,
    pub name: String,
    pub type_id: ScopeTypeId,
    pub mutable: bool,
    pub scope_id: ScopeId,
}

pub struct ScopeManager {
    next_type_id: ScopeTypeId,
    next_var_id: ScopeVarId,
    scopes: Vec<Scope>,
    types: Vec<ScopedType>,
    vars: Vec<ScopedVar>,
    pub cur: ScopeId,
}

impl ScopeManager {
    pub fn new() -> Self {
        let global_scope = Scope {
            id: 0,
            parent: None,
        };
        let cur_scope = Scope {
            id: 1,
            parent: Some(global_scope.id),
        };
        let types = vec![
            ScopedType {
                id: UNIT_TYPE_ID,
                parent_id: None,
                name: "Unit".to_string(),
                scope_id: global_scope.id,
            },
            ScopedType {
                id: INT_TYPE_ID,
                parent_id: None,
                name: "Int".to_string(),
                scope_id: global_scope.id,
            },
            ScopedType {
                id: FLOAT_TYPE_ID,
                parent_id: None,
                name: "Float".to_string(),
                scope_id: global_scope.id,
            },
            ScopedType {
                id: BOOL_TYPE_ID,
                parent_id: None,
                name: "Bool".to_string(),
                scope_id: global_scope.id,
            },
        ];
        Self {
            next_type_id: types.len() + 1,
            next_var_id: 0,
            cur: cur_scope.id,
            scopes: vec![global_scope, cur_scope],
            types,
            vars: vec![],
        }
    }

    pub fn create_scope(&mut self, parent: ScopeId) -> ScopeId {
        let id = self.scopes.len();
        let scope = Scope::child(id, parent);
        self.scopes.push(scope);
        id
    }

    pub fn lookup_type(&self, ty_name: &str, scope_id: ScopeId) -> Option<&ScopedType> {
        let scope = self.scopes.iter().find(|s| s.id == scope_id)?;
        if let Some(var) = self
            .types
            .iter()
            .find(|t| t.name == ty_name && t.scope_id == scope_id)
        {
            Some(var)
        } else if let Some(parent_id) = scope.parent {
            self.lookup_type(ty_name, parent_id)
        } else {
            None
        }
    }

    pub fn get_type(&self, ty_id: ScopeTypeId, scope_id: ScopeId) -> Option<&ScopedType> {
        let scope = self.scopes.iter().find(|s| s.id == scope_id)?;
        if let Some(ty) = self
            .types
            .iter()
            .find(|t| t.id == ty_id && t.scope_id == scope_id)
        {
            Some(ty)
        } else if let Some(parent_id) = scope.parent {
            self.get_type(ty_id, parent_id)
        } else {
            None
        }
    }

    pub fn is_child_type(&self, child_id: ScopeTypeId, parent_id: ScopeTypeId) -> bool {
        let child_type = self.types.iter().find(|t| t.id == child_id);
        if let Some(child_type) = child_type {
            if child_type.id == parent_id {
                return true;
            }
            if let Some(child_parent_id) = child_type.parent_id {
                return self.is_child_type(child_parent_id, parent_id);
            }
        }
        false
    }

    pub fn lookup_var(&self, var_name: &str, scope_id: ScopeId) -> Option<&ScopedVar> {
        let scope = self.scopes.iter().find(|s| s.id == scope_id)?;
        if let Some(var) = self
            .vars
            .iter()
            .find(|v| v.name == var_name && v.scope_id == scope_id)
        {
            Some(var)
        } else if let Some(parent_id) = scope.parent {
            self.lookup_var(var_name, parent_id)
        } else {
            None
        }
    }

    pub fn add_var(&mut self, var_name: &str, ty_id: ScopeTypeId, mutable: bool) {
        let var = ScopedVar {
            id: self.next_var_id,
            name: var_name.to_string(),
            type_id: ty_id,
            mutable,
            scope_id: self.cur,
        };
        self.vars.push(var);
        self.next_var_id += 1;
    }

    pub fn add_type(&mut self, ty_name: &str, parent_id: ScopeTypeId) {
        let ty = ScopedType {
            id: self.next_type_id,
            name: ty_name.to_string(),
            parent_id: Some(parent_id),
            scope_id: self.cur,
        };
        self.types.push(ty);
        self.next_type_id += 1;
    }
}
