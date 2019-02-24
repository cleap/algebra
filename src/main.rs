struct Operation {
    name: char,
}

fn op(name: char) -> Operation {
    Operation {
        name,
    }
}

struct Expression {
    val: String,
    terms: Vec<Expression>,
    op: char,
}

fn expr(str_rep: String, operations: &Vec<Operation>) -> Expression {

    let val = String::from(str_rep);
    let mut terms: Vec<Expression> = Vec::new();
    let mut op = '.';

    for operation in operations {
        let subexes: Vec<&str> = val.split(operation.name).collect();
        if subexes.len() > 1 {
            op = operation.name;
            for subex in subexes {
                terms.push(expr(String::from(subex), operations));
            }
            break;
        }
    }

    Expression {
        val,
        terms,
        op,
    }
}

fn print_ex(ex: &Expression) {
    if ex.terms.len() == 0 {
        print!("{}", ex.val);
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
        print!(")");
    }
    if is_evalable(&ex) {
        print!("_e ");
    } else {
        print!(" ");
    }
}

fn is_evalable(ex: &Expression) -> bool {
    if ex.terms.len() == 0 {
        !ex.val.parse::<f32>().is_err()
    } else {
        let mut res = true;
        for term in &ex.terms {
            if !is_evalable(term) {
                res = false
            }
        }
        res
    }
}

fn main() {
    let mut operations: Vec<Operation> = Vec::new();
    operations.push(op('='));
    operations.push(op('-'));
    operations.push(op('+'));
    operations.push(op('/'));
    operations.push(op('*'));
    operations.push(op('^'));

    let ex = expr(String::from("y*5+4/2=a*x^2+b*x+c+5*3"), &operations);
    print_ex(&ex);
    println!();
}
