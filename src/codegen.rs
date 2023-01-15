use crate::ast::{BinaryOperator, Expression, Program, Statement};

// entry
pub(crate) fn gen(node: Program) {
    let codegen = CodeGenerator::new(node);
    codegen.gen();
}

struct CodeGenerator {
    ast: Program,
}
impl CodeGenerator {
    fn new(ast: Program) -> Self {
        Self { ast }
    }
}
impl CodeGenerator {
    fn gen(&self) {
        for stmt in self.ast.statements.iter() {
            self.gen_stmt(stmt);
            println!("  # stack overflow prevention.");
            println!("  pop rax");
        }
    }

    fn gen_stmt(&self, node: &Statement) {
        match node {
            Statement::Expression(expr) => self.gen_expr(expr),
            Statement::Return(expr) => self.gen_return(expr),
            // _ => todo!(),
        }
    }

    fn gen_return(&self, node: &Expression) {
        self.gen_expr(node);
        println!("  pop rax");
        println!("  mov rsp, rbp");
        println!("  pop rbp");
        println!("  ret");
    }

    fn gen_expr(&self, node: &Expression) {
        match node {
            Expression::Integer(int) => {
                println!("  push {}", int);
            }
            Expression::LocalVariable { .. } => {
                self.gen_lval(node);
                println!("  pop rax");
                println!("  mov rax, [rax]");
                println!("  push rax");
            }
            Expression::Binary(bin) => {
                self.gen_expr(&*bin.lhs);
                self.gen_expr(&*bin.rhs);
                println!("  pop rdi");
                println!("  pop rax");
                match bin.op {
                    BinaryOperator::Plus => {
                        println!("  add rax, rdi");
                    }
                    BinaryOperator::Minus => {
                        println!("  sub rax, rdi");
                    }
                    BinaryOperator::Asterisk => {
                        println!("  imul rax, rdi");
                    }
                    BinaryOperator::Slash => {
                        println!("  cqo");
                        println!("  idiv rdi");
                    }
                    BinaryOperator::Lt => {
                        println!("  cmp rax, rdi");
                        println!("  setl al");
                        println!("  movzb rax, al");
                    }
                    BinaryOperator::LtEq => {
                        println!("  cmp rax, rdi");
                        println!("  setle al");
                        println!("  movzb rax, al");
                    }
                    BinaryOperator::Eq => {
                        println!("  cmp rax, rdi");
                        println!("  sete al");
                        println!("  movzb rax, al");
                    }
                    BinaryOperator::NotEq => {
                        println!("  cmp rax, rdi");
                        println!("  setne al");
                        println!("  movzb rax, al");
                    }
                    BinaryOperator::Assignment => {
                        self.gen_lval(&*bin.lhs);
                        self.gen_expr(&*bin.rhs);

                        println!("  pop rdi");
                        println!("  pop rax");
                        println!("  mov [rax], rdi");
                        println!("  push rdi");
                    } //
                      // _ => {
                      //     panic!("Invalid binary operator: {:?}", bin.op);
                      // }
                }
                println!("  push rax");
            }
            _ => {
                panic!("Invalid node: {:?}", node);
            }
        }
    }

    fn gen_lval(&self, node: &Expression) {
        match node {
            Expression::LocalVariable { offset, .. } => {
                println!("  mov rax, rbp");
                println!("  sub rax, {}", offset);
                println!("  push rax");
            }
            _ => {
                panic!(
                    "Invalid node: {:?}.\nleft node is not var on assignment expression.",
                    node
                );
            }
        }
    }
}
