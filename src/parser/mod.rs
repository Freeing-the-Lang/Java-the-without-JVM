use crate::ast::*;

pub fn parse_java(_src: &str) -> JavaClass {
    // Stub Parser (임시 파서)
    JavaClass {
        name: "Hello".into(),
        fields: vec![
            JavaField { name: "x".into(), value: 10 },
            JavaField { name: "y".into(), value: 22 },
        ],
        methods: vec![
            JavaMethod {
                name: "add".into(),
                body: JavaExpr::Return(Box::new(
                    JavaExpr::Add(
                        Box::new(JavaExpr::Int(10)),
                        Box::new(JavaExpr::Int(22)),
                    )
                )),
            }
        ]
    }
}
