use crate::{
    checking::globals::{TYPE_BOOL, TYPE_FLOAT, TYPE_INT, TYPE_UNIT},
    parsing::{expression::TypeIdent},
};

pub type ScopeId = usize;
pub type TypeId = usize;
pub type VariableId = usize;

pub struct Scope<'a> {
    pub id: ScopeId,
    pub parent: Option<ScopeId>,
    pub types: Vec<ScopedType<'a>>,
    pub vars: Vec<ScopedVariable>,
}

impl<'a> Scope<'a> {
    pub fn global() -> Self {
        Scope {
            id: 0,
            parent: None,
            types: vec![TYPE_INT, TYPE_FLOAT, TYPE_BOOL, TYPE_UNIT],
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

pub struct ScopedType<'a> {
    pub name: &'a str,
    pub id: TypeId,
}

pub struct ScopedVariable {
    pub name: String,
     pub id: VariableId,
    pub type_id: TypeId,
}

pub struct ScopeManager<'a> {
    scopes: Vec<Scope<'a>>,
    pub cur: ScopeId,
}

impl<'a> ScopeManager<'a> {
    pub fn new() -> Self {
        let global_scope = Scope::global();
        Self {
            cur: global_scope.id.clone(),
            scopes: vec![global_scope],
        }
    }

    fn create_scope(&mut self, parent: ScopeId) -> ScopeId {
        let id = self.scopes.len();
        let scope = Scope::child(id, parent);
        self.scopes.push(scope);
        id
    }

    fn lookup_type(&self, ty: &TypeIdent, scope_id: ScopeId) -> Option<&ScopedType> {
        let scope = &self.scopes[scope_id];
        if let Some(ty) = scope.types.iter().find(|t| t.name.eq(&ty.0)) {
            Some(ty)
        } else if let Some(parent_id) = scope.parent {
            self.lookup_type(ty, parent_id)
        } else {
            None
        }
    }

    pub fn lookup_var(&self, var_name: &str, scope_id: ScopeId) -> Option<&ScopedVariable> {
        let scope = &self.scopes[scope_id];
        if let Some(var) = scope.vars.iter().find(|v| v.name == var_name) {
            Some(var)
        } else if let Some(parent_id) = scope.parent {
            self.lookup_var(var_name, parent_id)
        } else {
            None
        }
    }

    fn add_var(
        &mut self,
        var_name: &str,
        var_type: TypeId,
    ) -> VariableId {
        let scope = match self.scopes.get_mut(self.cur) {
            Some(scope) => scope,
            None => panic!("Current scope does not exist"),
        };
        let variable_id = scope.vars.len();
        let var = ScopedVariable { name: var_name.to_string(), id: variable_id, type_id: var_type };
        scope.vars.push(var);
        variable_id
    }

    fn add_type(&mut self, ty_name: &'a str) -> TypeId {
        let scope = match self.scopes.get_mut(self.cur) {
            Some(scope) => scope,
            None => panic!("Current scope does not exist"),
        };
        let type_id = scope.types.len();
        let ty = ScopedType { name: ty_name, id: type_id };
        scope.types.push(ty);
        type_id
    }
}
