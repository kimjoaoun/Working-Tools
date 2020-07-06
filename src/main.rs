use webbrowser;
use std::{thread, time, process};


fn main() {
    println!("Mais um dia trabalho, vamos lá!\nComeçando em 5 segundos...");
    
    thread::sleep(time::Duration::from_secs(4));
    
    let paginas: Vec<&str> = vec![
        "https://wekan.mapeo.com.br/", 
        "https://lab.mapeo.com.br/", 
        "https://www.gitlab.com/",
        "https://app.asana.com/"];

    for i in paginas {
        webbrowser::open(i).expect("Erro ao abrir URL");
    }

    let output = process::Command::new("code")
            .status()
            .expect("Oops, não consegui iniciar o VSCode, ele está instalado?");


    println!("VSCode Status Code: {}", &output);

    thread::sleep(time::Duration::from_secs(5));

    process::exit(1);



}