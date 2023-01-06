use crate::parser::Expression;

pub fn analyze(exp: Expression) {
    todo!()

    // match exp {
    //     Expression::SelfEval => analyze_self_application(exp),
    //     Expression::Quoted => analyze_self_quoted(exp),
    //     Expression::Variable => analyze_self_variable(exp),
    //     Expression::Assignment => analyze_self_assignment(exp),
    //     Expression::Definition => analyze_self_definition(exp),
    //     Expression::If => analyze_self_if(exp),
    //     Expression::Lambda => analyze_self_lambda(exp),
    //     Expression::Begin => analyze_self_sequence(exp),
    //     Expression::Cond => analyze(cond_to_if(exp)),
    //     Expression::Application => analyze_self_application(exp),
    // }
}

pub fn analyze_self_evaluating(exp: Expression) {}
pub fn analyze_self_quoted(exp: Expression) {}
pub fn analyze_self_variable(exp: Expression) {}
pub fn analyze_self_assignment(exp: Expression) {}
pub fn analyze_self_definition(exp: Expression) {}
pub fn analyze_self_if(exp: Expression) {}
pub fn analyze_self_lambda(exp: Expression) {}
pub fn analyze_self_sequence(exp: Expression) {}
pub fn cond_to_if(exp: Expression) -> Expression {
    todo!()
}
pub fn analyze_self_application(exp: Expression) {}
