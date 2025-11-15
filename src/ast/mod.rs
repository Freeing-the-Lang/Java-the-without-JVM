#[derive(Debug, Clone)]
pub struct JavaClass {
    pub name: String,
    pub fields: Vec<JavaField>,
    pub methods: Vec<JavaMethod>,
}

#[derive(Debug, Clone)]
pub struct JavaField {
    pub name: String,
    pub value: i64,
}

#[derive(Debug, Clone)]
pub struct JavaMethod {
    pub name: String,
    pub body: JavaExpr,
}

#[derive(Debug, Clone)]
pub enum JavaExpr {
    Int(i64),
    Add(Box<JavaExpr>, Box<JavaExpr>),
    Return(Box<JavaExpr>),
}
