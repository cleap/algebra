struct Expression {
    val: String,
    terms: Vec<Expression>,
    op: char,
}

fn expr(val: String) -> Expression {
    let operations = ['=', '+', '-', '*', '/', '^'];

    let mut ex = Expression {
        val,
        terms: Vec::new(),
        op: '.',
    };

    for operation in &operations {
        let subexes: Vec<&str> = ex.val.split(*operation).collect();
        if subexes.len() > 1 {
            ex.op = *operation;
            for subex in subexes {
                ex.terms.push(expr(String::from(subex)));
            }
            break;
        }
    }
    ex
}

fn print_ex(ex: &Expression) {
    if ex.terms.len() == 0 {
        print!("{} ", ex.val);
    } else {
        print!("( ");
        let mut i = ex.terms.len();
        for term in &ex.terms {
            print_ex(term);
            if i > 1 {
                print!("{} ", ex.op);
            }
            i -= 1;
        }
        print!(") ");
    }
}

fn main() {
    println!("Hello, world!");
    let ex = expr(String::from("y*5+z^2=a*x^2+b*x+c"));
    print_ex(&ex);

}
