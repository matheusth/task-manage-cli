pub mod issue;

use issue::{Activity, Issue};

fn main() {
    let issue_one = Issue::new(
        String::from("test"),
        String::from("lorem ipsum dolor sit ammet"),
        14.2,
        String::from("22/07/1999"),
    );
    let issue_two = Issue::new(
        String::from("te"),
        String::from("Nova senha para o email profsubstituto.tri@ifgoiano.edu.br. Sou presidente da comissão de organização de processo seletivo e preciso dela para ter acesso aos documentos dos candidatos."),
        14.2,
        String::from("22/07/1999"),
    );

    let mut activity = Activity::new(
        String::from("Atendimento ao Usúrario"),
        String::from("descrição"),
        40.0,
    );

    activity.add_issue(issue_one);
    activity.add_issue(issue_two);
    activity.serialize();
}
