pub mod cnf_formula;
use cnf_formula::*;

pub fn find_propogatable(f:& Formula) -> Option<(Variable,bool)> {

    for clause in f {
        if clause.len() == 1 {
            let check = &clause[0];
            match check {
                Atom::Base(v) => return Some((*v, true)),  
                Atom::Not(v) => return Some((*v, false)), 
            }
        }
    }
    None
}

pub fn propogate_unit(f:& mut Formula,v:Variable,b:bool) {
    for clause in f.clone() {
        let mut tempt = clause.clone(); 

        if (b && tempt.contains(&Atom::Base(v))) || (!b && tempt.contains(&Atom::Not(v))) {
            f.retain(|c| c != &tempt);
        } else {
            tempt.retain(|atom| {
                match atom {
                    Atom::Base(var) => *var != v,  //v should be true here
                    Atom::Not(var) => *var != v,   
                    _ => true,
                }
            });

            let p = f.iter().position(|c| c == &clause).unwrap();
            f[p] = tempt;
        }
    }
}

pub fn find_pure_var(f:& Formula) -> Option<Variable> {
    let vars = get_vars(f); 
    for &v in &vars {
        if is_pure(f, v) {
            return Some(v);  
        }
    }
    None  
}

pub fn assign_pure_var(f: & mut Formula, v: Variable) {
    f.retain(|clause| {
        !(clause.contains(&Atom::Base(v)) ||  clause.contains(&Atom::Not(v)))
    });
}

pub fn unit_propogate(f:& mut Formula) {
    match find_propogatable(f) {
        Option::None => return,
        Option::Some((v,b)) => {
            propogate_unit(f, v, b);
            unit_propogate(f)
        }
    }
}

pub fn assign_pure_vars(f:& mut Formula) {
    match find_pure_var(f) {
        Option::None => return,
        Option::Some(v) => {
            assign_pure_var(f,v);
            assign_pure_vars(f); 
        }
    }
}

pub fn dpll(f:& mut Formula) -> bool {
    unit_propogate(f);
    assign_pure_vars(f);

    if f.is_empty() {
        return true;
    }

    if f.iter().any(|clause| clause.is_empty()) {
        return false;
    }
    let chr = match var_check(f) {
        Some(var) => var,
        None => return false, 
    };

    let mut case_true = f.clone();
    case_true.push(vec![Atom::Base(chr)]); 

    let mut case_false = f.clone();
    case_false.push(vec![Atom::Not(chr)]); 

    dpll(&mut case_true) || dpll(&mut case_false)
}

pub fn var_check(f: &Formula) -> Option<Variable> {
    for clause in f {
        for atom in clause {
            match atom {
                Atom::Base(v) | Atom::Not(v) => return Some(*v),
                _ => {}
            }
        }
    }

    None
}
